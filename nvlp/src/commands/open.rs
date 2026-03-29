use std::path::{Path, PathBuf};

use nvlp_core::github;

use super::decrypt_bytes;

pub fn handle(
    url: &str,
    identity: Option<&Path>,
    output: Option<&Path>,
    token: Option<&str>,
) -> anyhow::Result<()> {
    let gist_id = url.trim_end_matches('/').rsplit('/').next().unwrap();

    let token = github::resolve_token(token)?;
    let gist_file = github::fetch_gist(gist_id, &token)?;

    let plaintext = decrypt_bytes(gist_file.content.as_bytes(), identity)?;

    let output_path = match output {
        Some(p) => p.to_path_buf(),
        None => {
            let name = gist_file.filename;
            PathBuf::from(name.strip_suffix(".age").unwrap_or(&name).to_string())
        }
    };

    std::fs::write(&output_path, &plaintext)?;
    println!("Decrypted to {}", output_path.display());

    Ok(())
}
