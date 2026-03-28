use std::path::Path;

use envelop_core::{archive, crypto};

pub mod decrypt;
pub mod encrypt;
pub mod keys;
pub mod open;
pub mod send;

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
