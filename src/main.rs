mod args;
mod cmd;
mod http;
mod installer;
mod versions;

use std::env;

use args::{Cli, Commands};

use cmd::LsRemoteCommand;

use clap::Parser;

use installer::Installer;

fn main() {
    let buff = Installer::install_version("v0.5.0".to_string()).unwrap();
    Installer::unzip_version("v0.5.0".to_string(), buff);
    /*
    let args = Cli::parse();

    match args.command {
        Commands::Install { version } => todo!(),
        Commands::Use { version } => todo!(),
        Commands::Ls => todo!(),
        Commands::LsRemote => {
            LsRemoteCommand::list_remote_versions();
        }
        Commands::Current => todo!(),
        Commands::Uninstall { version } => todo!(),
    }
    */
}
