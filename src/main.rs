mod args;
mod cmd;
mod file_system;
mod http;
mod installer;
mod versions;

use args::{Cli, Commands};

use cmd::{InstallCommand, LsCommand};

use clap::Parser;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Install { version } => {
            InstallCommand::install_version(version);
        }
        Commands::Use { version } => todo!(),
        Commands::Ls => todo!(),
        Commands::LsRemote => {
            LsCommand::list_remote_versions();
        }
        Commands::Current => todo!(),
        Commands::Uninstall { version } => todo!(),
    }
}
