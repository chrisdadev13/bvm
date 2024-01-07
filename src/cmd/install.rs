use crate::file_system::FileSystem;
use crate::installer::Installer;
use crate::versions::Versions;

pub struct InstallCommand;
impl InstallCommand {
    pub fn install_version(version: String) {
        let already_installed = Versions::already_installed(version.clone());

        if already_installed {
            println!(
                "the bun version {} is already installed in the system",
                version.clone()
            );
            println!("do: bvm use {} instead", version.clone());
            return;
        }

        let bold_start = "\x1b[1m";
        let bold_end = "\x1b[0m";

        println!("{}bvm install{} {}", bold_start, bold_end, version.clone());
        Installer::install_version(version.clone());
        Installer::unzip_version(version.clone());
        Versions::clean_dir(version.clone());
        println!("   ✅ Done");
        FileSystem::create_symlink(version.clone());
    }
}
