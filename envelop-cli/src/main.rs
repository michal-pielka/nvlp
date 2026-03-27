use std::path::PathBuf;

use clap::Parser;
use envelop_cli::cli::{Args, Command};

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Send { file, to } => handle_send_command(file, &to),
    }
}

fn handle_send_command(file: PathBuf, to: &str) {}
