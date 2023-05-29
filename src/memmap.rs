use std::fs::File;

use memmap2::Mmap;

/// Try to memmap a file if:
/// - It is greater than 16 KiB
/// - Less than a isize's max value
///
/// Based on [the following code here](https://github.com/BLAKE3-team/BLAKE3/blob/71a2646180c787e22f8681c5fec7655a0ad51e99/b3sum/src/main.rs#LL248C1-L248C1)
/// from the BLAKE3 repository.
pub(crate) fn try_memmap(file: &File) -> anyhow::Result<Option<Mmap>> {
    let metadata = file.metadata()?;
    let file_size = metadata.len();

    const MIN_SIZE: u64 = 16 * 1024;
    const MAX_SIZE: u64 = isize::MAX as u64;

    if !metadata.is_file() || file_size == 0 || file_size > MAX_SIZE || file_size < MIN_SIZE {
        Ok(None)
    } else {
        // SAFETY: This is marked as unsafe as memmaps are always UB if the underlying file
        // is modified.
        //
        // We are safe however to have multiple mmaps at the same time internally, since
        // we only ever borrow immutably.
        let mmap = unsafe {
            memmap2::MmapOptions::new()
                .len(file_size as usize)
                .map(file)?
        };

        Ok(Some(mmap))
    }
}
