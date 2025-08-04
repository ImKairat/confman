use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "\"confman\" is a lightweight, command-line configuration manager for Linux systems.\n\nGitHub: https://github.com/ImKairat/confman",
    long_about = None,
    arg_required_else_help = true,
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Config {
        #[arg(short, long, default_value = "configuration.cm")]
        file_name: String,

        #[arg(short, long, default_value = "~/")]
        file_path: String

    },
    Init,
    Start {
        #[arg(short, long)]
        verbose: bool,
    },
    Sync {
        #[arg(short, long)]
        url: Option<String>,
    },
}
