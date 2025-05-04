use clap::ValueEnum;

use crate::TargetType;

// Limit is 128 KiB.
const SKIP_RAYON_LIMIT: usize = 128 * 1024;

/// The various hash functions.
#[derive(Default, Clone, Copy, Debug, ValueEnum)]
pub(crate) enum HashFunction {
    #[default]
    Blake3,
    #[cfg(feature = "blake2")]
    Blake2b,
    #[cfg(feature = "blake2")]
    Blake2s,
    #[cfg(feature = "md5")]
    MD5,
    #[cfg(feature = "sha1")]
    Sha1,
    #[cfg(feature = "sha2")]
    Sha2_256,
    #[cfg(feature = "sha2")]
    Sha2_384,
    #[cfg(feature = "sha2")]
    Sha2_512,
    #[cfg(feature = "sha3")]
    Sha3_256,
    #[cfg(feature = "sha3")]
    Sha3_384,
    #[cfg(feature = "sha3")]
    Sha3_512,
}

#[derive(Clone)]
enum InternalHasher {
    Blake3(Box<blake3::Hasher>),
    #[cfg(feature = "blake2")]
    Blake2b(Box<blake2::Blake2b512>),
    #[cfg(feature = "blake2")]
    Blake2s(Box<blake2::Blake2s256>),
    #[cfg(feature = "md5")]
    MD5(Box<md5::Context>),
    #[cfg(feature = "sha1")]
    Sha1(Box<sha1::Sha1>),
    #[cfg(feature = "sha2")]
    Sha2_256(Box<sha2::Sha256>),
    #[cfg(feature = "sha2")]
    Sha2_384(Box<sha2::Sha384>),
    #[cfg(feature = "sha2")]
    Sha2_512(Box<sha2::Sha512>),
    #[cfg(feature = "sha3")]
    Sha3_256(Box<sha3::Sha3_256>),
    #[cfg(feature = "sha3")]
    Sha3_384(Box<sha3::Sha3_384>),
    #[cfg(feature = "sha3")]
    Sha3_512(Box<sha3::Sha3_512>),
}

impl InternalHasher {
    /// Update the internal state of the hasher given some bytes.
    fn update(&mut self, bytes: &[u8]) {
        #[cfg(feature = "sha1")]
        use sha1::digest::Update;

        match self {
            InternalHasher::Blake3(h) => {
                if bytes.len() >= SKIP_RAYON_LIMIT {
                    h.update_rayon(bytes);
                } else {
                    h.update(bytes);
                }
            }
            #[cfg(feature = "blake2")]
            InternalHasher::Blake2b(h) => h.update(bytes),
            #[cfg(feature = "blake2")]
            InternalHasher::Blake2s(h) => h.update(bytes),
            #[cfg(feature = "md5")]
            InternalHasher::MD5(ctx) => ctx.consume(bytes),
            #[cfg(feature = "sha1")]
            InternalHasher::Sha1(h) => h.update(bytes),
            #[cfg(feature = "sha2")]
            InternalHasher::Sha2_256(h) => h.update(bytes),
            #[cfg(feature = "sha2")]
            InternalHasher::Sha2_384(h) => h.update(bytes),
            #[cfg(feature = "sha2")]
            InternalHasher::Sha2_512(h) => h.update(bytes),
            #[cfg(feature = "sha3")]
            InternalHasher::Sha3_256(h) => h.update(bytes),
            #[cfg(feature = "sha3")]
            InternalHasher::Sha3_384(h) => h.update(bytes),
            #[cfg(feature = "sha3")]
            InternalHasher::Sha3_512(h) => h.update(bytes),
        }
    }

    /// Finalize the hash computation and return a hash.
    fn finalize(self) -> Vec<u8> {
        #[cfg(feature = "sha1")]
        use sha1::digest::FixedOutput;

        match self {
            InternalHasher::Blake3(h) => h.finalize().as_bytes().to_vec(),
            #[cfg(feature = "blake2")]
            InternalHasher::Blake2b(h) => h.finalize_fixed().to_vec(),
            #[cfg(feature = "blake2")]
            InternalHasher::Blake2s(h) => h.finalize_fixed().to_vec(),
            #[cfg(feature = "md5")]
            InternalHasher::MD5(ctx) => ctx.compute().0.to_vec(),
            #[cfg(feature = "sha1")]
            InternalHasher::Sha1(h) => h.finalize_fixed().to_vec(),
            #[cfg(feature = "sha2")]
            InternalHasher::Sha2_256(h) => h.finalize_fixed().to_vec(),
            #[cfg(feature = "sha2")]
            InternalHasher::Sha2_384(h) => h.finalize_fixed().to_vec(),
            #[cfg(feature = "sha2")]
            InternalHasher::Sha2_512(h) => h.finalize_fixed().to_vec(),
            #[cfg(feature = "sha3")]
            InternalHasher::Sha3_256(h) => h.finalize_fixed().to_vec(),
            #[cfg(feature = "sha3")]
            InternalHasher::Sha3_384(h) => h.finalize_fixed().to_vec(),
            #[cfg(feature = "sha3")]
            InternalHasher::Sha3_512(h) => h.finalize_fixed().to_vec(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct DircsHasher {
    state: InternalHasher,
}

impl DircsHasher {
    pub(crate) fn new(hash_function: HashFunction) -> Self {
        #[cfg(feature = "sha1")]
        use sha1::digest::Digest;

        let hasher = match hash_function {
            HashFunction::Blake3 => InternalHasher::Blake3(blake3::Hasher::new().into()),
            #[cfg(feature = "blake2")]
            HashFunction::Blake2b => InternalHasher::Blake2b(blake2::Blake2b512::new().into()),
            #[cfg(feature = "blake2")]
            HashFunction::Blake2s => InternalHasher::Blake2s(blake2::Blake2s256::new().into()),
            #[cfg(feature = "md5")]
            HashFunction::MD5 => InternalHasher::MD5(md5::Context::new().into()),
            #[cfg(feature = "sha1")]
            HashFunction::Sha1 => InternalHasher::Sha1(sha1::Sha1::new().into()),
            #[cfg(feature = "sha2")]
            HashFunction::Sha2_256 => InternalHasher::Sha2_256(sha2::Sha256::new().into()),
            #[cfg(feature = "sha2")]
            HashFunction::Sha2_384 => InternalHasher::Sha2_384(sha2::Sha384::new().into()),
            #[cfg(feature = "sha2")]
            HashFunction::Sha2_512 => InternalHasher::Sha2_512(sha2::Sha512::new().into()),
            #[cfg(feature = "sha3")]
            HashFunction::Sha3_256 => InternalHasher::Sha3_256(sha3::Sha3_256::new().into()),
            #[cfg(feature = "sha3")]
            HashFunction::Sha3_384 => InternalHasher::Sha3_384(sha3::Sha3_384::new().into()),
            #[cfg(feature = "sha3")]
            HashFunction::Sha3_512 => InternalHasher::Sha3_512(sha3::Sha3_512::new().into()),
        };

        Self { state: hasher }
    }

    pub(crate) fn hash_result(mut self, bytes_vec: &[(usize, Vec<u8>)]) -> Vec<u8> {
        for (_, bytes) in bytes_vec {
            self.state.update(bytes);
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

                if total_bytes >= SKIP_RAYON_LIMIT {
                    hasher.update_rayon(cursor.get_ref());
                } else {
                    hasher.update(cursor.get_ref());
                }

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
                            self.state.update(&buffer[..bytes_read]);
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
