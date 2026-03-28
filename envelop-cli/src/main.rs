use std::path::{Path, PathBuf};

use clap::Parser;
use envelop_cli::cli::{Args, Command};

use envelop_core::{archive, crypto, github};

fn main() -> anyhow::Result<()> {
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
            identity,
            output,
        } => handle_open_command(&url, identity.as_deref(), &output),

        Command::Encrypt { files, to, output } => {
            handle_encrypt_command(&files, &to, output.as_deref())
        }

        Command::Decrypt {
            file,
            identity,
            output,
        } => handle_decrypt_command(&file, identity.as_deref(), &output),

        Command::Keys { username } => handle_keys_command(&username),
    }
}

fn handle_send_command(
    paths: &[PathBuf],
    to: &str,
    description: Option<&str>,
    comment: Option<&str>,
    token: Option<&str>,
) -> anyhow::Result<()> {
    // Fetch friend's public keys
    let public_keys = github::fetch_public_keys(to)?;
    let public_keys: Vec<&str> = public_keys.iter().map(|k| k.as_str()).collect();

    // Pack file(s) into tar archive
    let payload = archive::pack_files(paths)?;

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

fn handle_encrypt_command(
    paths: &[PathBuf],
    to: &str,
    output: Option<&Path>,
) -> anyhow::Result<()> {
    let public_keys = github::fetch_public_keys(to)?;
    let public_keys: Vec<&str> = public_keys.iter().map(|k| k.as_str()).collect();

    let payload = archive::pack_files(paths)?;
    let ciphertext = crypto::encrypt(&payload, &public_keys)?;

    let output_path = match output {
        Some(p) => p.to_path_buf(),
        None if paths.len() == 1 => {
            let mut name = paths[0].file_name().unwrap().to_os_string();
            name.push(".age");
            PathBuf::from(name)
        }
        None => PathBuf::from("envelop.age"),
    };

    std::fs::write(&output_path, &ciphertext)?;
    println!("Encrypted to {}", output_path.display());

    Ok(())
}

fn decrypt_and_unpack(
    ciphertext: &[u8],
    identity: Option<&Path>,
    output: &Path,
) -> anyhow::Result<()> {
    let identity = match identity {
        Some(p) => p.to_path_buf(),
        None => dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("could not find home directory"))?
            .join(".ssh/id_ed25519"),
    };
    let private_key = std::fs::read_to_string(identity)?;
    let plaintext = crypto::decrypt(ciphertext, &private_key, None)?;
    archive::unpack_files(&plaintext, output)?;
    Ok(())
}

fn handle_open_command(url: &str, identity: Option<&Path>, output: &Path) -> anyhow::Result<()> {
    let parts: Vec<&str> = url.trim_end_matches('/').rsplit('/').collect();
    let gist_id = parts[0];
    let owner = parts[1];

    let ciphertext = github::download_gist_content(gist_id, owner)?;
    decrypt_and_unpack(ciphertext.as_bytes(), identity, output)
}

fn handle_decrypt_command(
    file: &Path,
    identity: Option<&Path>,
    output: &Path,
) -> anyhow::Result<()> {
    let ciphertext = std::fs::read(file)?;
    decrypt_and_unpack(&ciphertext, identity, output)
}

fn handle_keys_command(username: &str) -> anyhow::Result<()> {
    // Fetch friend's public keys
    let public_keys = github::fetch_public_keys(username)?;

    for key in &public_keys {
        println!("{key}");
    }

    Ok(())
}
