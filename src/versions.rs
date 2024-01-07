use crate::file_system::FileSystem;

use dirs::home_dir;
use std::fs::{remove_file, rename};
use std::path::Path;

pub struct Versions;
impl Versions {
    pub fn switch(version: String) {
        if self::Versions::already_installed(version.clone()) {
            FileSystem::create_symlink(version.clone());
        } else {
            println!("The version {} doesn't exist in the system.", version);
            println!("Do: bvm install {} instead", version);
        }
    }
    pub fn already_installed(version: String) -> bool {
        let home_dir = home_dir().unwrap().join(".bvm/");
        let version_dir = home_dir.join(version);

        Path::new(&version_dir).is_dir()
    }

    pub fn clean_dir(version: String) {
        let home_dir = dirs::home_dir();

        if let Some(dir) = home_dir {
            // File to rename
            let from = dir.join(format!(".bvm/{}/bun-darwin-x64", version));
            // New name
            let to = dir.join(format!(".bvm/{}/bin", version));
            rename(from, to).expect("  ⚠️  Cleaning failed renaming the folder");

            let zip = dir.join(format!(".bvm/{}/bun.zip", version));
            remove_file(zip).expect("  ⚠️  Cleaning failed removing the zip file");
        }
    }
}
