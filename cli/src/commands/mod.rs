use std::path::Path;

use nvlp_core::{crypto, github};

pub mod decrypt;
pub mod encrypt;
pub mod keys;
pub mod open;
pub mod send;

fn fetch_all_keys(recipients: &[String]) -> anyhow::Result<Vec<String>> {
    anyhow::ensure!(
        !recipients.is_empty(),
        "at least one --to recipient is required"
    );
    let mut all_keys = Vec::new();
    for username in recipients {
        let keys = github::fetch_public_keys(username)?;
        anyhow::ensure!(
            !keys.is_empty(),
            "user @{username} has no SSH keys on GitHub"
        );
        all_keys.extend(keys);
    }
    Ok(all_keys)
}

fn decrypt_bytes(ciphertext: &[u8], identity: Option<&Path>) -> anyhow::Result<Vec<u8>> {
    let identity = match identity {
        Some(p) => p.to_path_buf(),
        None => dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("could not find home directory"))?
            .join(".ssh/id_ed25519"),
    };
    let private_key = std::fs::read_to_string(identity)?;
    Ok(crypto::decrypt(ciphertext, &private_key, None)?)
}
