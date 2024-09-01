use std::{env, process::Command};

const HASH_LIST: [&str; 11] = [
    "blake3", "blake2b", "blake2s", "md5", "sha1", "sha2-256", "sha2-384", "sha2-512", "sha3-256",
    "sha3-384", "sha3-512",
];

fn dircs() -> Command {
    let exe = env!("CARGO_BIN_EXE_dircs");
    Command::new(exe)
}

fn get_hash(path: &str) -> String {
    let out = dircs().arg(path).output().unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    let hash = stdout.rsplit_once(' ').unwrap().1;

    hash.strip_suffix('\n').unwrap().to_string()
}

fn hash_with_fn(path: &str, f: &str) -> String {
    let out = dircs().arg(path).arg("-f").arg(f).output().unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    let hash = stdout.rsplit_once(' ').unwrap().1;

    hash.strip_suffix('\n').unwrap().to_string()
}

fn print_hash_with_fn(path: &str, f: &str) {
    let result = hash_with_fn(path, f);
    println!("{f}: {result}");
}

#[test]
fn dir_checksum() {
    assert_eq!(
        get_hash("./tests/test_dir"),
        "da3e3c42d529bc285a65ff1bfe6a220e31c631185afabde6e63dffa4b1c55142"
    );
}

#[test]
fn sub_dir_checksum() {
    assert_eq!(
        get_hash("./tests/test_dir/sub_dir"),
        "d364677c85f04e475fc6a041d8cb4c54c4dcc3d93161d162d8db8c89f7598749"
    );
}

#[test]
fn file_checksum() {
    assert_eq!(
        get_hash("./tests/test_dir/a.txt"),
        "81c4b7f7e0549f1514e9cae97cf40cf133920418d3dc71bedbf60ec9bd6148cb"
    );
}

#[test]
fn different_file_different_hashes() {
    let a = get_hash("./tests/test_dir/a.txt");
    let b = get_hash("./tests/test_dir/b.txt");

    assert_ne!(a, b);
}

#[test]
fn empty_dir() {
    let out = dircs().arg("./tests/test_dir/empty_dir").output().unwrap();
    assert!(String::from_utf8(out.stdout)
        .unwrap()
        .contains("no files to hash"));
}

#[test]
fn test_hashers_basic() {
    for hash_fn in HASH_LIST {
        print_hash_with_fn("./tests/test_dir/a.txt", hash_fn);
    }
}

#[test]
fn all_hashers_same_twice() {
    for hash_fn in HASH_LIST {
        let first_try = hash_with_fn("./tests/test_dir/a.txt", hash_fn);
        let second_try = hash_with_fn("./tests/test_dir/a.txt", hash_fn);

        assert_eq!(
            first_try, second_try,
            "dircs should return the same result for the same file when run multiple times"
        );
    }
}

#[test]
fn all_hashers_diff_twice() {
    for hash_fn in HASH_LIST {
        let first_file = hash_with_fn("./tests/test_dir/a.txt", hash_fn);
        let second_file = hash_with_fn("./tests/test_dir/b.txt", hash_fn);

        assert_ne!(
            first_file, second_file,
            "dircs should return a different hash for different files"
        );
    }
}

#[test]
fn multiple_inputs() {
    let out = dircs()
        .args(vec![
            "./tests/test_dir/a.txt",
            "./tests/test_dir/sub_dir",
            "./tests/test_dir/b.txt",
            "./tests/test_dir/empty_dir",
        ])
        .output()
        .unwrap();

    let result = String::from_utf8(out.stdout).unwrap();

    let correct = "\
./tests/test_dir/a.txt -> 81c4b7f7e0549f1514e9cae97cf40cf133920418d3dc71bedbf60ec9bd6148cb
./tests/test_dir/b.txt -> 9d902f9864f3043dca97e40698eee07a2fe6771591c687ed129cde8f6fcc4a79
./tests/test_dir/empty_dir -> there were no files to hash
./tests/test_dir/sub_dir -> d364677c85f04e475fc6a041d8cb4c54c4dcc3d93161d162d8db8c89f7598749\n";

    assert_eq!(result, correct);
}
