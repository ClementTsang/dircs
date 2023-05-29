mod hash_functions;

mod memmap;

mod args;

use std::{
    fs::File,
    io::{Cursor, Read},
    time::Instant,
};

use anyhow::bail;
use args::*;

use clap::Parser;
use hash_functions::DircsHasher;
use jwalk::WalkDir;
use memmap::try_memmap;

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

fn get_path_hash(args: &Args) -> anyhow::Result<Vec<u8>> {
    let mut walker = WalkDir::new(&args.path)
        .sort(true)
        .skip_hidden(args.skip_hidden);

    if let Some(max_threads) = args.max_threads {
        walker = walker.parallelism(jwalk::Parallelism::RayonNewPool(max_threads));
    }

    if let Some(depth) = args.depth {
        walker = walker.max_depth(depth);
    }

    let hasher = DircsHasher::new(args.hash);

    let mut file_hash_results = walker
        .into_iter()
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path().canonicalize().unwrap();
                if path.is_dir() {
                    return None;
                }

                if args.verbose {
                    println!("Processing {path:?}");
                }

                let file = File::open(&path).unwrap();

                let target = if args.memmap {
                    match try_memmap(&file) {
                        Ok(Some(mmap)) => TargetType::MMap(Cursor::new(mmap)),
                        _ => TargetType::File(file),
                    }
                } else {
                    TargetType::File(file)
                };

                match hasher.clone().hash_target(target) {
                    Ok((result, bytes_read)) => {
                        if args.verbose {
                            let hex = hex::encode(&result);
                            println!("{path:?} -> {hex} ({bytes_read} bytes read)",);
                        }
                        Some(result)
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

    if file_hash_results.is_empty() {
        bail!("There were no successful hashes computed!");
    } else if file_hash_results.len() == 1 {
        Ok(file_hash_results.pop().unwrap())
    } else {
        Ok(hasher.hash_bytes(&file_hash_results))
    }
}

fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let args = Args::parse();
    let hash = get_path_hash(&args)?;

    let hex = hex::encode(hash);
    let path = args.path.to_string_lossy();

    println!("{path} -> {hex}");
    if args.verbose {
        println!("Took {}s.", start.elapsed().as_secs());
    }

    Ok(())
}
