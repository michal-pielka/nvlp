use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use nvlp_core::crypto;

use super::fetch_all_keys;

pub fn handle(file: Option<&Path>, to: &[String], output: Option<&Path>) -> anyhow::Result<()> {
    let public_keys = fetch_all_keys(to)?;
    let public_keys: Vec<&str> = public_keys.iter().map(|k| k.as_str()).collect();

    let payload = match file {
        Some(path) => std::fs::read(path)?,
        None => {
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf)?;
            buf
        }
    };

    let ciphertext = crypto::encrypt(&payload, &public_keys)?;

    match output {
        Some(p) => {
            std::fs::write(p, &ciphertext)?;
            eprintln!("Encrypted to {}", p.display());
        }
        None => match file {
            Some(path) => {
                let mut name = path.file_name().unwrap().to_os_string();
                name.push(".age");
                let output_path = PathBuf::from(name);
                std::fs::write(&output_path, &ciphertext)?;
                eprintln!("Encrypted to {}", output_path.display());
            }
            None => {
                io::stdout().write_all(&ciphertext)?;
            }
        },
    }

    Ok(())
}
