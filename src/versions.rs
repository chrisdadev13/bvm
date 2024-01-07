use dirs::home_dir;
use std::fs::{remove_file, rename};
use std::path::Path;

pub struct Versions;
impl Versions {
    pub fn already_installed(version: String) -> bool {
        let home_dir = home_dir().unwrap().join(".bvm/");
        let version_dir = home_dir.join(version);

        Path::new(&version_dir).is_dir()
    }

    pub fn clean_dir(version: String) {
        let home_dir = dirs::home_dir();

        if let Some(dir) = home_dir {
            let zip = dir.join(format!(".bvm/{}/bun.zip", version));

            // File to rename
            let from = dir.join(format!(".bvm/{}/bun-darwin-x64", version));
            // New name
            let to = dir.join(format!(".bvm/{}/bin", version));

            rename(from, to).unwrap_err();
            remove_file(zip).unwrap_err();
        }
    }
}
