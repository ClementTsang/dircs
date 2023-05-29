# dircs

A utility to get a hash of a file/directory, with a variety of hash functions to choose from.

**Note**: This is a program I mostly wrote for personal use, and there might be a lot of rough edges! Feel free to
submit pull requests or bug reports.

## Installation

`dircs` is installable using `cargo`:

```bash
cargo install dircs --locked
```

As of writing, `dircs` is built using Rust 1.69.0.

## Usage

```bash
dircs /your/path/here
```

By default, `dircs` will use BLAKE3 with memmapping disabled, and an automatically chosen level of parallelism.

See more options by running `dircs -h`.
