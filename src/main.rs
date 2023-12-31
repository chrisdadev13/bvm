mod args;
mod cmd;
mod http;

use args::{Cli, Commands};

use cmd::LsRemoteCommand;

use clap::Parser;

fn main() {
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
}
