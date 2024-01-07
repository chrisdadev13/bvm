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

        Installer::install_version(version.clone());
        Installer::unzip_version(version.clone());
        Versions::clean_dir(version.clone());
    }
}
