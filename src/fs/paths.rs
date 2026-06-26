use dirs::data_local_dir;
use std::path::PathBuf;

use crate::config::constants::{
    ANT_BIN_DIRECTORY, ANT_COMMAND, BASE_DIRECTORY, BINARY_DIRECTORY, SEVEN_ZIP_DIRECTOY,
};

pub fn get_base_directory() -> PathBuf {
    let mut base_dir = data_local_dir().unwrap();
    base_dir.push(BASE_DIRECTORY);
    base_dir
}

pub fn get_binary_directory() -> PathBuf {
    let mut binary_dir = get_base_directory();
    binary_dir.push(BINARY_DIRECTORY);
    binary_dir
}

pub fn get_ant_executable() -> PathBuf {
    let mut ant_dir = get_binary_directory();
    ant_dir.push(ANT_BIN_DIRECTORY);
    ant_dir.push(ANT_COMMAND);
    ant_dir
}

pub fn get_7zip_executable() -> PathBuf {
    let mut seven_dir = get_binary_directory();
    seven_dir.push(SEVEN_ZIP_DIRECTOY);
    seven_dir
}
