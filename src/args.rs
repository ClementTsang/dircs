use std::path::PathBuf;

use clap::Parser;

use crate::hash_functions::HashFunction;

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
        help = "Whether to skip hidden files.",
        default_value = "false"
    )]
    pub skip_hidden: bool,

    #[arg(
        short,
        long,
        help = "Whether to enable memmapping for reading files. Note this will not always be used, even if enabled.",
        default_value = "false"
    )]
    pub memmap: bool,

    #[arg(help = "The path to run the program on.")]
    pub path: PathBuf,
}
