# dircs

[<img src="https://img.shields.io/crates/v/dircs.svg?style=flat-square" alt="crates.io link">](https://crates.io/crates/dircs)

A small cross-platform utility to get the hash of a file or directory. Comes with a variety of hash functions to choose
from.

**Note**: I mostly wrote this for my own personal use, so there might be a lot of rough edges. Feel free to submit
pull requests, feature requests, or bug reports, but my time working on this may be limited.

## Installation

### Cargo

`dircs` is on [crates.io](https://crates.io/crates/dircs) and can be installed with `cargo`:

```bash
cargo install dircs --locked
```

As of writing, `dircs` has been tested to build using Rust 1.70.0.

If you want to disable certain hash functions for whatever reason, you can control this by disabling the appropriate
feature for that hash function (see `Cargo.toml`).

### Binaries

As of 0.1.9, binaries are generated on release through CI. These can be accessed in the
[releases](https://github.com/ClementTsang/dircs/releases), with the latest releases found
[here](https://github.com/ClementTsang/dircs/releases/latest).

To use them, download the appropriate binary for your system and run the binary as needed.

## Usage

```bash
$ dircs /your/path/here
/your/path/here -> 72ce3b5f2df28051cf7204712fe93de6b7b6d1f8e8fe5972b117a248423c290c
```

By default, `dircs` will use BLAKE3 with memmapping disabled, and an automatic level of parallelism.

See more options by running `dircs -h`.

### Supported hash functions

Currently, the following hash functions are supported:

- BLAKE3 (default)
- BLAKE2
- MD5
- SHA1
- SHA2 (digest sizes of 256, 384, 512)
- SHA3 (digest sizes of 256, 384, 512)

## How are hashes determined?

For a single file or a directory with just one file, `dircs` will simply hash the file using the specified hash
function, and output the bytes as a hex string.

For a directory with multiple files, we do the following:

1. For every file, get the hash using the chosen hashing function. This occurs in parallel if possible.
2. With this generated list of hashes, sort them based on file name.
3. Feed each hash in this order into the chosen hash function to generate one final hash, which is then output as a hex
   string.

## Thanks

Thanks to:

- All library authors whose libraries I used.
- The authors of [b3sum](https://github.com/BLAKE3-team/BLAKE3/tree/master/b3sum), which I referenced a lot.
