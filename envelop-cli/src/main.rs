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

        Command::Open {
            url,
            identity_path,
            output_path,
        } => handle_open_command(&url, &identity_path, &output_path),
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

fn handle_open_command(
    url: &str,
    identity_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse url
    let parts: Vec<&str> = url.trim_end_matches('/').rsplit('/').collect();
    let gist_id = parts[0];
    let owner = parts[1];

    // Download gist content: envelop.age
    let ciphertext = github::download_gist_content(gist_id, owner)?;

    // Read private key
    let private_key = std::fs::read_to_string(identity_path)?;

    // Decrypt
    let plaintext_bytes = crypto::decrypt(ciphertext.as_bytes(), &private_key, None)?;

    // TODO: hardcoded path - we want to maintain single file filename
    if archive::unpack_files(&plaintext_bytes, output_path).is_err() {
        std::fs::write(output_path.join("envelop.out"), &plaintext_bytes)?;
    }

    Ok(())
}
