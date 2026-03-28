use std::path::PathBuf;

use clap::Parser;
use envelop_cli::cli::{Args, Command};

use envelop_core::{crypto, github};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Command::Send { file, to, token } => handle_send_command(file, &to, token.as_deref()),
    }
}

fn handle_send_command(
    file: PathBuf,
    to: &str,
    token: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch friend's public keys
    let public_keys = github::fetch_public_keys(to)?;
    let public_keys: Vec<&str> = public_keys.iter().map(|k| k.as_str()).collect();

    // Read the file
    let plaintext = std::fs::read(file)?;

    // Encrypt to those public keys
    let ciphertext_bytes = crypto::encrypt(&plaintext, &public_keys)?;
    let ciphertext = std::str::from_utf8(&ciphertext_bytes)?;

    // Resolve token
    let token = github::resolve_token(token)?;

    // Create the gist
    let gist = github::create_gist(ciphertext, to, &token)?;

    // Comment on the gist
    github::comment_on_gist(&gist, to, None, &token)?;

    println!("Secret sent to @{to}\n{}", gist.html_url);

    Ok(())
}
