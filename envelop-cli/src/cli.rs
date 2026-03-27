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
        file: PathBuf,

        #[arg(short, long, value_name = "USERNAME")]
        to: String,
    },
}
