use std::path::{Path, PathBuf};

use nvlp_core::github;

use super::decrypt_bytes;

pub fn handle(url: &str, identity: Option<&Path>, output: Option<&Path>) -> anyhow::Result<()> {
    let parts: Vec<&str> = url.trim_end_matches('/').rsplit('/').collect();
    let gist_id = parts[0];
    let owner = parts[1];

    let ciphertext = github::download_gist_content(gist_id, owner)?;
    let plaintext = decrypt_bytes(ciphertext.as_bytes(), identity)?;

    let output_path = match output {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("nvlp"),
    };

    std::fs::write(&output_path, &plaintext)?;
    println!("Decrypted to {}", output_path.display());

    Ok(())
}
