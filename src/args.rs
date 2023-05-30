use std::path::PathBuf;

use clap::Parser;

use crate::hashers::HashFunction;

/// The arguments for the program.
#[derive(Parser)]
pub(crate) struct Args {
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
        help = "The maximum recursion depth to scan. Will scan as deep as possible by default."
    )]
    pub depth: Option<usize>,

    #[arg(
        short = 't',
        long,
        help = "The maximum number of CPU threads to use, if possible. Automatically chosen by default."
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
        help = "Whether to enable memmapping for reading files. Off by default. May use a lot of memory.",
        long_help = "Whether to enable memmapping for reading files. Off by default. Note that this use a lot of memory, especially with multiple threads, and that some files will skip this option, even if enabled. Also note that if a memmapped file is modified, there may be undefined behaviour!",
        default_value = "false"
    )]
    pub memmap: bool,

    #[arg(help = "The path to run the dircs on. Can be a file or directory")]
    pub path: PathBuf,

    #[arg(
        short,
        long,
        help = "Whether to be verbose. Off by default.",
        default_value = "false"
    )]
    pub verbose: bool,
}
