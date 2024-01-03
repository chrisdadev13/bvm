use dirs::home_dir;
use std::fs::create_dir;
use std::path::Path;

use crate::http;

pub struct Installer;
impl Installer {
    pub fn install_version(version: String) {
        self::Installer::create_versions_dir();

        create_dir(dirs::home_dir().unwrap().join(format!(".bvm/{}", version)));

        let version_dir = dirs::home_dir()
            .unwrap()
            .join(format!(".bvm/{}/bun.zip", version));

        let version_path = version_dir.as_path();

        match http::HTTPRequest::download_zip(
            format!(
                "https://github.com/oven-sh/bun/releases/download/bun-{}/bun-darwin-x64.zip",
                version
            )
            .as_str(),
            version_path,
        ) {
            Ok(_) => {
                println!("Success");
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
    pub fn create_versions_dir() {
        let versions_dir = home_dir().unwrap().join(".bvm");

        if Path::new(&versions_dir).is_dir() {
            return;
        }

        create_dir(versions_dir).expect("failed to create versions directory");
    }
}
