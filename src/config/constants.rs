use include_dir::{Dir, include_dir};

pub const BINARIES: Dir = include_dir!("$CARGO_MANIFEST_DIR/binaries");
pub const BINARIES_SIZE: u64 = 5522982;

pub const BASE_DIRECTORY: &str = "Lib Builder";
pub const BINARY_DIRECTORY: &str = "binaries";
pub const ANT_BIN_DIRECTORY: &str = "apache-ant/bin";
pub const ANT_COMMAND: &str = if cfg!(windows) { "ant.bat" } else { "ant" };
pub const SEVEN_ZIP_DIRECTOY: &str = "7zip/7z.exe";
pub const CREATE_NO_WINDOW_FLAG: u32 = 0x08000000;
pub const FILE_TO_DELETE: &str = "META-INF/persistence.xml";
