use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "TODO")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Send {
        #[arg(required = true, num_args = 1..)]
        files: Vec<PathBuf>,

        #[arg(short, long, value_name = "USERNAME")]
        to: String,

        #[arg(short, long)]
        description: Option<String>,

        #[arg(short, long)]
        comment: Option<String>,

        #[arg(long, value_name = "TOKEN")]
        token: Option<String>,
    },

    Open {
        url: String,

        #[arg(short, long)]
        identity_path: Option<PathBuf>,

        #[arg(short, long, default_value = ".")]
        output_path: PathBuf,
    },

    Keys {
        username: String,
    },
}
