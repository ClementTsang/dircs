use clap::ValueEnum;
use sha1::Digest;

use crate::TargetType;

/// The various hash functions.
#[derive(Default, Clone, Copy, Debug, ValueEnum)]
pub(crate) enum HashFunction {
    #[default]
    Blake3,
    Sha1,
    Sha2_256,
    Sha2_512,
    Sha3_256,
    Sha3_512,
    MD5,
}

#[derive(Clone)]
enum InternalHasher {
    Blake3(blake3::Hasher),
    Sha1(sha1::Sha1),
    Sha2_256(sha2::Sha256),
    Sha2_512(sha2::Sha512),
    Sha3_256(sha3::Sha3_256),
    Sha3_512(sha3::Sha3_512),
    MD5(md5::Context),
}

impl InternalHasher {
    /// Update the internal state of the hasher given some bytes.
    fn update(&mut self, bytes: &[u8], use_rayon: bool) {
        match self {
            InternalHasher::Blake3(h) => {
                if use_rayon {
                    h.update_rayon(bytes);
                } else {
                    h.update(bytes);
                }
            }
            InternalHasher::Sha1(h) => {
                h.update(bytes);
            }
            InternalHasher::Sha2_256(h) => {
                h.update(bytes);
            }
            InternalHasher::Sha2_512(h) => {
                h.update(bytes);
            }
            InternalHasher::Sha3_256(h) => {
                h.update(bytes);
            }
            InternalHasher::Sha3_512(h) => {
                h.update(bytes);
            }
            InternalHasher::MD5(ctx) => {
                ctx.consume(bytes);
            }
        }
    }

    /// Finalize the hash computation and return a hash.
    fn finalize(self) -> Vec<u8> {
        match self {
            InternalHasher::Blake3(h) => h.finalize().as_bytes().to_vec(),
            InternalHasher::Sha1(h) => h.finalize().to_vec(),
            InternalHasher::Sha2_256(h) => h.finalize().to_vec(),
            InternalHasher::Sha2_512(h) => h.finalize().to_vec(),
            InternalHasher::Sha3_256(h) => h.finalize().to_vec(),
            InternalHasher::Sha3_512(h) => h.finalize().to_vec(),
            InternalHasher::MD5(ctx) => ctx.compute().0.to_vec(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct DircsHasher {
    state: InternalHasher,
}

impl DircsHasher {
    pub(crate) fn new(hash_function: HashFunction) -> Self {
        let hasher = match hash_function {
            HashFunction::Blake3 => InternalHasher::Blake3(blake3::Hasher::new()),
            HashFunction::Sha1 => InternalHasher::Sha1(sha1::Sha1::new()),
            HashFunction::Sha2_256 => InternalHasher::Sha2_256(sha2::Sha256::new()),
            HashFunction::Sha2_512 => InternalHasher::Sha2_512(sha2::Sha512::new()),
            HashFunction::Sha3_256 => InternalHasher::Sha3_256(sha3::Sha3_256::new()),
            HashFunction::Sha3_512 => InternalHasher::Sha3_512(sha3::Sha3_512::new()),
            HashFunction::MD5 => InternalHasher::MD5(md5::Context::new()),
        };

        Self { state: hasher }
    }

    pub(crate) fn hash_bytes(mut self, bytes_vec: &[Vec<u8>]) -> Vec<u8> {
        for bytes in bytes_vec {
            self.state.update(bytes, false);
        }
        self.state.finalize()
    }

    pub(crate) fn hash_target(
        mut self,
        mut target: TargetType,
    ) -> anyhow::Result<(Vec<u8>, usize)> {
        match (&target, &mut self.state) {
            (TargetType::MMap(cursor), InternalHasher::Blake3(hasher)) => {
                // If we have memmap and blake3 enabled, we can use this nifty feature!

                let total_bytes = cursor.get_ref().len();
                hasher.update_rayon(cursor.get_ref());
                Ok((hasher.finalize().as_bytes().to_vec(), total_bytes))
            }
            _ => {
                const BUFFER_SIZE: usize = 64 * 1024; // 64 KiB buffer size.
                let mut buffer = [0; BUFFER_SIZE];
                let mut total_bytes = 0;

                loop {
                    match target.read(&mut buffer) {
                        Ok(0) => {
                            return Ok((self.state.finalize(), total_bytes));
                        }
                        Ok(bytes_read) => {
                            total_bytes += bytes_read;
                            self.state.update(&buffer[..bytes_read], false);
                        }
                        Err(err) => {
                            if err.kind() == std::io::ErrorKind::Interrupted {
                                continue;
                            } else {
                                return Err(err.into());
                            }
                        }
                    }
                }
            }
        }
    }
}
