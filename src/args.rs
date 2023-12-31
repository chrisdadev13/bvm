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
    /// download and activate the specified bun version
    #[command(arg_required_else_help = true)]
    Install {
        /// the specified bun version to install
        version: String,
    },

    /// activate the specified bun version in the current shell
    #[command(arg_required_else_help = true)]
    Use {
        /// the specified bun version to use
        version: String,
    },

    /// list installed bun versions
    Ls,

    /// list available bun versions to install
    LsRemote,

    /// print the currently-active Node version
    Current,

    /// uninstall the specified bun version
    #[command(arg_required_else_help = true)]
    Uninstall {
        /// the specified Bun version to uninstall
        version: String,
    },
}
