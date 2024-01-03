use dirs::home_dir;
use flate2::read::GzDecoder;
use std::error::Error;
use std::fs::{create_dir, File};
use std::io::{self, Read, Write};
use std::path::Path;
use zip::ZipArchive;

use crate::http;

pub struct Installer;
impl Installer {
    pub fn unzip_version(version: String, buffer: Vec<u8>) {
        let cwd = dirs::home_dir();
        let mut decoder = GzDecoder::new(&buffer[..]);
        let mut decompressed = Vec::new();

        decoder.read_to_end(&mut decompressed).unwrap();

        let mut file = File::create(cwd.unwrap().join(format!(".bvm/{}/bun", version))).unwrap();
        file.write_all(&decompressed).unwrap();
    }
    pub fn install_version(version: String) -> Result<Vec<u8>, Box<dyn Error>> {
        self::Installer::create_versions_dir();

        let _ = create_dir(dirs::home_dir().unwrap().join(format!(".bvm/{}", version)));

        let version_dir = dirs::home_dir()
            .unwrap()
            .join(format!(".bvm/{}/bun.zip", version));

        let temp_zip_file = version_dir.as_path();

        match http::HTTPRequest::download_zip(
            format!(
                "https://github.com/oven-sh/bun/releases/download/bun-{}/bun-darwin-x64.zip",
                version
            )
            .as_str(),
            temp_zip_file,
        ) {
            Ok(buffer) => {
                println!("Success");
                Ok(buffer)
            }
            Err(e) => {
                eprintln!("{:?}", e);
                Err(e)
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
