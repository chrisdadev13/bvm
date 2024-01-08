use dirs::home_dir;

pub struct UninstallCommand;
impl UninstallCommand {
    pub fn uninstall(version: String) {
        let home_directory = home_dir();

        if let Some(home) = home_directory {
            let source = home.join(".bvm").join(version);
            if source.exists() {
                let _ = std::fs::remove_dir_all(source);
                println!("Done!!");
            }
        }
    }
}
