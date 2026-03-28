use std::path::PathBuf;

use clap::Parser;
use envelop_cli::cli::{Args, Command};

use envelop_core::{archive, crypto, github};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Command::Send {
            files,
            to,
            description,
            comment,
            token,
        } => handle_send_command(
            &files,
            &to,
            description.as_deref(),
            comment.as_deref(),
            token.as_deref(),
        ),
    }
}

fn handle_send_command(
    paths: &[PathBuf],
    to: &str,
    description: Option<&str>,
    comment: Option<&str>,
    token: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch friend's public keys
    let public_keys = github::fetch_public_keys(to)?;
    let public_keys: Vec<&str> = public_keys.iter().map(|k| k.as_str()).collect();

    // Pack file(s)
    let payload = match paths.len() {
        1 => std::fs::read(&paths[0])?,
        _ => archive::pack_files(paths)?,
    };

    // Encrypt to those public keys
    let ciphertext_bytes = crypto::encrypt(&payload, &public_keys)?;
    let ciphertext = std::str::from_utf8(&ciphertext_bytes)?;

    // Resolve token
    let token = github::resolve_token(token)?;

    // Create the gist
    let gist = github::create_gist(ciphertext, to, description, &token)?;

    // Comment on the gist
    github::comment_on_gist(&gist, to, comment, &token)?;

    println!("Secret sent to @{to}\n{}", gist.html_url);

    Ok(())
}
