use dirs::home_dir;
use std::fs::{create_dir, create_dir_all, set_permissions, File, Permissions};
use std::io::copy;
use std::io::{self, Write};
use std::path::Path;

use crate::http;

pub struct Installer;
impl Installer {
    pub fn unzip_version(version: String) {
        println!("   📦 Unzipping [2/3]");
        let file_zip = dirs::home_dir()
            .unwrap()
            .join(format!(".bvm/{}/bun.zip", version));

        let file = File::open(file_zip.as_path()).unwrap();

        let mut archive = zip::ZipArchive::new(file).unwrap();
        let destination_directory = dirs::home_dir().unwrap().join(format!(".bvm/{}/", version));

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();

            let file_path = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };
            let mut outpath = destination_directory.clone();
            outpath.push(file_path);

            {
                let comment = file.comment();
                if !comment.is_empty() {}
            }

            if (*file.name()).ends_with('/') {
                create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = File::create(&outpath).unwrap();
                copy(&mut file, &mut outfile).unwrap();
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    set_permissions(&outpath, Permissions::from_mode(mode)).unwrap();
                }
            }
        }
    }
    pub fn install_version(version: String) {
        self::Installer::create_versions_dir();

        let _ = create_dir(dirs::home_dir().unwrap().join(format!(".bvm/{}", version)));

        let version_dir = dirs::home_dir()
            .unwrap()
            .join(format!(".bvm/{}/bun.zip", version));

        let temp_zip_file = version_dir.as_path();

        println!("   🔎 Resolving [1/3]");
        match http::HTTPRequest::download_zip(
            format!(
                "https://github.com/oven-sh/bun/releases/download/bun-{}/bun-darwin-x64.zip",
                version
            )
            .as_str(),
            temp_zip_file,
        ) {
            Ok(_) => {
                io::stdout().flush().unwrap();
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
