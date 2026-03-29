use std::io::{self, Read, Write};
use std::path::Path;

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
        None => {
            io::stdout().write_all(&plaintext)?;
        }
    }

    Ok(())
}
