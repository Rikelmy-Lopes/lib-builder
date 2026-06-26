use walkdir::WalkDir;

use crate::{
    config::constants::{BINARIES, BINARIES_SIZE},
    fs::paths::get_binary_directory,
};

pub fn extract_binaries() {
    let binary_dir = get_binary_directory();

    if get_extract_binaries_size() != BINARIES_SIZE {
        let _ = BINARIES.extract(binary_dir);
    }
}

pub fn get_extract_binaries_size() -> u64 {
    WalkDir::new(get_binary_directory())
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| entry.metadata().ok())
        .map(|meta| meta.len())
        .sum()
}
