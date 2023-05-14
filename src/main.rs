use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Default, Clone, Copy, Debug, ValueEnum)]
enum HashFunction {
    #[default]
    Blake3,
    Sha1,
    Sha2_256,
    Sha2_512,
    Sha3_256,
    Sha3_512,
}

#[derive(Parser)]
struct Args {
    #[arg(
        short = 'f',
        long,
        help = "The hash function to use.",
        default_value = "blake3"
    )]
    hash: HashFunction,

    #[arg(
        short,
        long,
        help = "The maximum recursion depth to scan. Infinite by default."
    )]
    depth: Option<u64>,

    #[arg(
        short,
        long,
        help = "The maximum number of CPU threads to use, if possible. Uses all by default."
    )]
    max_threads: u64,

    #[arg(help = "The path to run the program on.")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
}
