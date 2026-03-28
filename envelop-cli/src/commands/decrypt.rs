use std::path::Path;

use super::decrypt_and_unpack;

pub fn handle(file: &Path, identity: Option<&Path>, output: &Path) -> anyhow::Result<()> {
    let ciphertext = std::fs::read(file)?;
    decrypt_and_unpack(&ciphertext, identity, output)
}
