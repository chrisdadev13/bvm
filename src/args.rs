use clap::{Parser, Subcommand};

/// bvm - bun version manager
#[derive(Debug, Parser)]
#[command(name = "bvm")]
#[command(about="Bun version manager", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Download and activate the specified Bun version
    #[command(arg_required_else_help = true)]
    Install {
        /// The specified Bun version to install
        version: String,
    },

    /// Activate the specified Node version in the current shell
    #[command(arg_required_else_help = true)]
    Use {
        /// The specified Bun version to use
        version: String,
    },

    /// List installed Bun versions
    Ls,

    /// List available Bun versions to install
    LsRemote,

    /// Print the currently-active Bun version
    Current,

    /// Uninstall the specified Bun version
    #[command(arg_required_else_help = true)]
    Uninstall {
        /// The specified Bun version to uninstall
        version: String,
    },
}
