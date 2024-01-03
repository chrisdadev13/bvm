use dirs::home_dir;
use std::path::Path;

pub struct Versions;
impl Versions {
    fn already_installed(version: String) -> bool {
        let home_dir = home_dir().unwrap().join(".bvm/");
        let version_dir = home_dir.join(version);

        Path::new(&version_dir).is_dir()
    }
}
