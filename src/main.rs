//! dircs is a  small cross-platform utility to get
//! the hash of a file or directory.

mod args;
mod hashers;
mod memmap;

#[cfg(feature = "progress")]
mod progress;

use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
    time::Instant,
};

use args::*;

#[cfg(feature = "progress")]
use progress::ProgressBarState;

use anyhow::bail;
use clap::Parser;
use hashers::DircsHasher;
use jwalk::WalkDir;
use memmap::try_memmap;
use rayon::{
    ThreadPoolBuilder,
    prelude::{ParallelBridge, ParallelIterator},
};

enum TargetType {
    MMap(Cursor<memmap2::Mmap>),
    File(File),
}

impl TargetType {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        match self {
            TargetType::MMap(cursor) => cursor.read(buffer),
            TargetType::File(file) => file.read(buffer),
        }
    }
}

fn get_path_hash(args: &Args, path: &Path) -> anyhow::Result<Vec<u8>> {
    let mut walker = WalkDir::new(path).sort(true).skip_hidden(args.skip_hidden);

    if args.verbose {
        if args.skip_hidden {
            println!("Skipping hidden files.");
        } else {
            println!("Not skipping hidden files.");
        }
    }

    if let Some(max_threads) = args.max_threads {
        ThreadPoolBuilder::new()
            .num_threads(max_threads)
            .thread_name(|i| format!("dircs-thread-{i}"))
            .build_global()?;

        if max_threads == 1 {
            walker = walker.parallelism(jwalk::Parallelism::Serial);
        } else {
            walker = walker.parallelism(jwalk::Parallelism::RayonNewPool(max_threads));
        }

        if args.verbose {
            if max_threads == 1 {
                println!("Using {max_threads} thread for file traversal.");
            } else {
                println!("Using {max_threads} threads for file traversal.");
            }
        }
    }

    if let Some(depth) = args.depth {
        walker = walker.max_depth(depth);

        if args.verbose {
            println!("Setting a max depth of {depth} for file traversal.");
        }
    }

    let hasher = DircsHasher::new(args.hash);

    #[cfg(feature = "progress")]
    let progress_bar_state = (args.progress && !args.verbose).then(ProgressBarState::default);

    let mut file_hash_results = walker
        .into_iter()
        .enumerate()
        .par_bridge()
        .filter_map(|(index, entry)| {
            if let Ok(entry) = entry {
                let Ok(path) = entry.path().canonicalize() else {
                    if args.verbose {
                        println!(
                            "{} no longer exists, skipping",
                            entry.path().to_string_lossy()
                        );
                    }
                    return None;
                };

                if path.is_dir() {
                    return None;
                }

                let Ok(file) = File::open(&path) else {
                    if args.verbose {
                        println!("{} cannot be opened, skipping", path.to_string_lossy());
                    }
                    return None;
                };

                let target = if args.memmap {
                    match try_memmap(&file) {
                        Ok(Some(mmap)) => TargetType::MMap(Cursor::new(mmap)),
                        _ => TargetType::File(file),
                    }
                } else {
                    TargetType::File(file)
                };

                #[cfg(feature = "progress")]
                if let Some(progress_bar_state) = &progress_bar_state {
                    progress_bar_state.update_length();

                    if let Some(index) = rayon::current_thread_index() {
                        progress_bar_state.set_thread_progress(index, &path)
                    }
                }

                let hash_result = hasher.clone().hash_target(target);

                #[cfg(feature = "progress")]
                if let Some(progress_bar_state) = &progress_bar_state {
                    if let Some(index) = rayon::current_thread_index() {
                        progress_bar_state.finish_thread_progress(index)
                    }

                    progress_bar_state.update_progress();
                }

                match hash_result {
                    Ok((result, bytes_read)) => {
                        if args.verbose {
                            let hex = hex::encode(&result);
                            println!("{path:?} -> {hex} ({bytes_read} bytes read)",);
                        }
                        Some((index, result))
                    }
                    Err(err) => {
                        if args.verbose {
                            println!("Couldn't process {path:?} because: `{err:?}`. Skipping.");
                        }
                        None
                    }
                }
            } else {
                if args.verbose {
                    println!("Found an issue with entry {entry:?}, skipping.");
                }

                None
            }
        })
        .collect::<Vec<_>>();

    #[cfg(feature = "progress")]
    if let Some(progress_bar_state) = progress_bar_state {
        progress_bar_state.finish();
    }

    if file_hash_results.is_empty() {
        bail!("there were no files to hash");
    } else if file_hash_results.len() == 1 {
        Ok(file_hash_results.pop().unwrap().1)
    } else {
        // Sort by index, compute final results.
        file_hash_results.sort_by_key(|(index, _)| *index);
        Ok(hasher.hash_result(&file_hash_results))
    }
}

fn verify_args(args: &Args) -> anyhow::Result<()> {
    if let Some(max_threads) = args.max_threads {
        if max_threads == 0 {
            bail!("max_threads must be 1 or greater!");
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let mut args = Args::parse();

    verify_args(&args)?;
    args.sort_args();

    for path in &args.paths {
        match get_path_hash(&args, path) {
            Ok(hash) => {
                let hex = hex::encode(hash);
                let path = path.to_string_lossy();

                println!("{path} -> {hex}");
            }
            Err(err) => {
                let path = path.to_string_lossy();

                if args.verbose {
                    println!("{path} -> {err}");
                    println!("backtrace: \n{err:?}");
                } else {
                    println!("{path} -> {err}");
                }
            }
        }

        if args.verbose {
            println!("Took {:.3}s.", start.elapsed().as_secs_f64());
        }
    }

    Ok(())
}
