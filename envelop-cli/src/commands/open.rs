use std::path::Path;

use envelop_core::github;

use super::decrypt_and_unpack;

pub fn handle(url: &str, identity: Option<&Path>, output: &Path) -> anyhow::Result<()> {
    let parts: Vec<&str> = url.trim_end_matches('/').rsplit('/').collect();
    let gist_id = parts[0];
    let owner = parts[1];

    let ciphertext = github::download_gist_content(gist_id, owner)?;
    decrypt_and_unpack(ciphertext.as_bytes(), identity, output)
}
