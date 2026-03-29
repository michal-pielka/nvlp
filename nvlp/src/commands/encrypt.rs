use std::path::{Path, PathBuf};

use nvlp_core::crypto;

use super::fetch_all_keys;

pub fn handle(file: &Path, to: &[String], output: Option<&Path>) -> anyhow::Result<()> {
    let public_keys = fetch_all_keys(to)?;
    let public_keys: Vec<&str> = public_keys.iter().map(|k| k.as_str()).collect();

    let payload = std::fs::read(file)?;
    let ciphertext = crypto::encrypt(&payload, &public_keys)?;

    let output_path = match output {
        Some(p) => p.to_path_buf(),
        None => {
            let mut name = file.file_name().unwrap().to_os_string();
            name.push(".age");
            PathBuf::from(name)
        }
    };

    std::fs::write(&output_path, &ciphertext)?;
    println!("Encrypted to {}", output_path.display());

    Ok(())
}
