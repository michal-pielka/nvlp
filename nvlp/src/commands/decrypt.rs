use std::path::{Path, PathBuf};

use super::decrypt_bytes;

pub fn handle(file: &Path, identity: Option<&Path>, output: Option<&Path>) -> anyhow::Result<()> {
    let ciphertext = std::fs::read(file)?;
    let plaintext = decrypt_bytes(&ciphertext, identity)?;

    let output_path = match output {
        Some(p) => p.to_path_buf(),
        None => {
            let name = file.file_name().unwrap().to_string_lossy();
            PathBuf::from(name.strip_suffix(".age").unwrap_or(&name).to_string())
        }
    };

    std::fs::write(&output_path, &plaintext)?;
    println!("Decrypted to {}", output_path.display());

    Ok(())
}
