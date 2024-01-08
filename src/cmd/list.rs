use std::fs;

use dirs::home_dir;

use crate::file_system::FileSystem;
use crate::http::HTTPRequest;

pub struct LsCommand;
impl LsCommand {
    pub fn list() {
        let home_folder = home_dir();
        if let Some(home) = home_folder {
            let path = home.join(".bvm");
            let directories = fs::read_dir(path).expect("~/bvm directory not found");

            for entry in directories {
                let entry = entry.expect("version directory not found");
                let path = entry.path();
                let version = FileSystem::get_last_component_of_path(path.clone());

                if path.is_dir() {
                    println!("   {}", version.unwrap());
                }
            }
        }
    }
    pub fn list_remote_versions() {
        match HTTPRequest::bun_registry() {
            Ok(res) => {
                for tag in &res[11..res.len() - 8] {
                    let version = tag.ref_field.to_string()[14..].to_string();
                    println!("   {}", version);
                }
            }
            Err(_) => {
                eprint!("error fetching remote versions")
            }
        }
    }
}
