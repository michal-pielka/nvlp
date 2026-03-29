use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use super::decrypt_bytes;

pub fn handle(
    file: Option<&Path>,
    identity: Option<&Path>,
    output: Option<&Path>,
) -> anyhow::Result<()> {
    let ciphertext = match file {
        Some(path) => std::fs::read(path)?,
        None => {
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf)?;
            buf
        }
    };

    let plaintext = decrypt_bytes(&ciphertext, identity)?;

    match output {
        Some(p) => {
            std::fs::write(p, &plaintext)?;
            eprintln!("Decrypted to {}", p.display());
        }
        None => match file {
            Some(path) => {
                let name = path.file_name().unwrap().to_string_lossy();
                let output_path =
                    PathBuf::from(name.strip_suffix(".age").unwrap_or(&name).to_string());
                std::fs::write(&output_path, &plaintext)?;
                eprintln!("Decrypted to {}", output_path.display());
            }
            None => {
                io::stdout().write_all(&plaintext)?;
            }
        },
    }

    Ok(())
}
