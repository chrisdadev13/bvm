use dirs::home_dir;

use std::os::unix::fs as unix_fs;
use std::path::PathBuf;

pub struct FileSystem;
impl FileSystem {
    pub fn create_symlink(version: String) {
        let home_directory = home_dir();

        if let Some(home) = home_directory {
            let source = home.join(".bvm").join(version).join("bin").join("bun");
            let destination = PathBuf::from("/usr/local/bin/bun");
            if destination.exists() {
                std::fs::remove_file(&destination).unwrap();
            }

            unix_fs::symlink(source, destination).expect("Error linking the new version");
        }
    }
}
