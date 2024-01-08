mod args;
mod cmd;
mod file_system;
mod http;
mod installer;
mod versions;

use args::{Cli, Commands};

use cmd::{InstallCommand, LsCommand, UninstallCommand, Use};

use clap::Parser;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Install { version } => {
            InstallCommand::install_version(version);
        }
        Commands::Use { version } => {
            Use::use_command(version);
        }
        Commands::Ls => {
            LsCommand::list();
        }
        Commands::LsRemote => {
            LsCommand::list_remote_versions();
        }
        Commands::Uninstall { version } => {
            UninstallCommand::uninstall(version);
        }
    }
}
