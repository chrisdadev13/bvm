use crate::file_system::FileSystem;
use crate::versions::Versions;

pub struct Use;
impl Use {
    pub fn use_command(version: String) {
        if Versions::already_installed(version.clone()) {
            FileSystem::create_symlink(version.clone());
            println!("Now using Bun {}", version);
        } else {
            println!("The version {} doesn't exist in the system.", version);
            println!("Do: bvm install {} instead", version);
        }
    }
}
