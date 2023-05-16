use clap::ValueEnum;

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
}

impl HashFunction {
    pub(crate) fn hash(&self, data: &[u8]) {
        match self {
            HashFunction::Blake3 => todo!(),
            HashFunction::Sha1 => todo!(),
            HashFunction::Sha2_256 => todo!(),
            HashFunction::Sha2_512 => todo!(),
            HashFunction::Sha3_256 => todo!(),
            HashFunction::Sha3_512 => todo!(),
        }
    }
}
