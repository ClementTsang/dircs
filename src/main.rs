mod hash_functions;

mod args;

use std::fs::File;

use args::*;

use clap::Parser;
use jwalk::WalkDir;
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

fn get_path_hash(args: &Args) -> anyhow::Result<&[u8]> {
    let mut walker = WalkDir::new(&args.path)
        .sort(true)
        .skip_hidden(args.skip_hidden);

    if let Some(max_threads) = args.max_threads {
        walker = walker.parallelism(jwalk::Parallelism::RayonNewPool(max_threads));
    }

    if let Some(depth) = args.depth {
        walker = walker.max_depth(depth);
    }

    let hash_list = walker
        .into_iter()
        .par_bridge()
        .into_par_iter()
        .map(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path().canonicalize().unwrap();
                let file = File::open(path).unwrap();

                if args.memmap {
                    // TODO: Do this.
                } else {
                }
            }
        })
        .reduce(|| (), |acc, op| {});

    Ok(todo!())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let hash = get_path_hash(&args)?;

    println!("{}", args.path.to_string_lossy());

    Ok(())
}
