# dircs

[<img src="https://img.shields.io/crates/v/dircs.svg?style=flat-square" alt="crates.io link">](https://crates.io/crates/dircs)

A small cross-platform utility to get the hash of a file or directory. Comes with a variety of hash functions to choose from.

**Note**: I mostly wrote this for personal use, and there might be a lot of rough edges! Feel free to submit pull
requests or bug reports.

## Installation

`dircs` is on [crates.io](https://crates.io/crates/dircs) and is installable using `cargo`:

```bash
cargo install dircs --locked
```

As of writing, `dircs` is built using Rust 1.69.0.

## Usage

```bash
$ dircs /your/path/here
/your/path/here -> 72ce3b5f2df28051cf7204712fe93de6b7b6d1f8e8fe5972b117a248423c290c
```

By default, `dircs` will use BLAKE3 with memmapping disabled, and an automatically chosen level of parallelism.

See more options by running `dircs -h`.

### Supported hash functions

Currently, the following hash functions are supported:

- BLAKE3
- MD5
- SHA1
- SHA2 (digest sizes of 256, 384, 512)
- SHA3 (digest sizes of 256, 384, 512)

## Thanks

Thanks to:

- All library authors whose libraries I used.
- The authors of [b3sum](https://github.com/BLAKE3-team/BLAKE3/tree/master/b3sum), which I referenced a lot.
