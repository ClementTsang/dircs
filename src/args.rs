use std::path::PathBuf;

use clap::Parser;

use crate::hashers::HashFunction;

/// Arguments for dircs.
#[derive(Parser)]
#[command(about = "A small cross-platform utility to get a hash of a file or directory.")]
#[command(version)]
pub(crate) struct Args {
    #[arg(help = "The path to run the dircs on. Can be a file or directory")]
    pub path: PathBuf,

    #[arg(
        short = 'f',
        long,
        help = "The hash function to use.",
        default_value = "blake3"
    )]
    pub hash: HashFunction,

    #[arg(
        short,
        long,
        help = "The maximum recursion depth for file traversal. Will scan as deep as possible by default."
    )]
    pub depth: Option<usize>,

    #[arg(
        short = 't',
        long,
        help = "The maximum number of CPU threads to use. Automatically chosen by default.",
        long_help = "The maximum number of CPU threads to use. Automatically chosen by default. Used for parallelizing file hashing and directory traversal."
    )]
    pub max_threads: Option<usize>,

    #[arg(
        short,
        long,
        help = "Whether to skip hidden files. Does not skip by default.",
        default_value = "false"
    )]
    pub skip_hidden: bool,

    #[arg(
        short,
        long,
        help = "Whether to enable memmapping for reading files. Disabled by default. May use a lot of memory.",
        long_help = "Whether to enable memmapping for reading files. Disabled by default. Note that:
* This uses a lot of memory, especially with multiple threads.
* This does not necessarily result in speedups.
* Some files will not be memmapped if the file is too small or too large.
* If a memmapped file is modified, there may be undefined behaviour!",
        default_value = "false"
    )]
    pub memmap: bool,

    #[arg(
        short,
        long,
        help = "Whether to show verbose logging. Disabled by default.",
        default_value = "false"
    )]
    pub verbose: bool,
}
