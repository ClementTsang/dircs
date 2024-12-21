use std::path::PathBuf;

use clap::Parser;

use crate::hashers::HashFunction;

/// Arguments for dircs.
#[derive(Parser)]
#[command(about = "A small cross-platform utility to get the hash of a file or directory.")]
#[command(version)]
#[command(arg_required_else_help = true)]
pub(crate) struct Args {
    #[arg(
        num_args(1..),
        help = "The paths to run the dircs on.",
        long_help = "The paths to run the dircs on. Can be a file or directory. Multiple paths can be specified."
    )]
    pub paths: Vec<PathBuf>,

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
        default_value_t = false
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
        default_value_t = false
    )]
    pub memmap: bool,

    #[cfg(feature = "progress")]
    #[arg(
        short = 'p',
        long,
        help = "Enable a progress bar. Will not show if verbose is enabled.",
        default_value_t = false
    )]
    pub progress: bool,

    #[arg(
        short,
        long,
        help = "Whether to show verbose logging. Disabled by default.",
        default_value_t = false
    )]
    pub verbose: bool,
}

impl Args {
    pub(crate) fn sort_args(&mut self) {
        self.paths.sort();
    }
}
