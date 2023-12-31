mod args;

use args::{Cli, Commands};
use clap::Parser;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Install { version } => todo!(),
        Commands::Use { version } => todo!(),
        Commands::Ls => todo!(),
        Commands::LsRemote => todo!(),
        Commands::Current => todo!(),
        Commands::Uninstall { version } => todo!(),
    }
}
