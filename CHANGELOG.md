# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-05-04

### Other

- Use rayon with blake3 mode if input is larger than 128 KiB.

## [0.2.0] - 2024-12-20

### Features

- Add a `-p`/`--progress` option to show a progress bar while dircs is working.

## [0.1.10] - 2024-09-03

### Other

- Update a bunch of dependencies.

## [0.1.9] - 2024-05-22

### Features

- Show help menu when passing in no arguments, rather than doing nothing.

## [0.1.8] - 2023-11-19

### Features

- BLAKE2 support.

## [0.1.7] - 2023-06-22

### Features

- Support multiple input paths.

## [0.1.6] - 2023-06-07

### Other

- Update the README.
- Exclude some files in the distributed release on crates.io.

## [0.1.5] - 2023-06-05

### Other

- Just a few small optimizations and additional tests.

## [0.1.4] - 2023-05-31

### Features

- Add `-V`/`--version` to the help options.

## [0.1.3] - 2023-05-31

### Bug Fixes

- Fixes help menu's application description.

## [0.1.2] - 2023-05-30

### Bug Fixes

- Fixes some typos in the help menu.

## [0.1.1] - 2023-05-30

### Features

- Added 384-bit digest size hashes.

### Other

- Updated documentation.
- Added tests.

## [0.1.0] - 2023-05-29

Initial release.
