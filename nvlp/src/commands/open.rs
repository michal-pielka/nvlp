use std::io::Write;
use std::path::{Path, PathBuf};

use nvlp_core::github;

use super::decrypt_bytes;

pub fn handle(
    url: &str,
    identity: Option<&Path>,
    output: Option<&Path>,
    stdout: bool,
    token: Option<&str>,
) -> anyhow::Result<()> {
    let gist_id = url
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("invalid gist URL: {url}"))?;

    let token = github::resolve_token(token)?;
    let gist_file = github::fetch_gist(gist_id, &token)?;

    let plaintext = decrypt_bytes(gist_file.content.as_bytes(), identity)?;

    if stdout {
        std::io::stdout().write_all(&plaintext)?;
        return Ok(());
    }

    let output_path = match output {
        Some(p) => p.to_path_buf(),
        None => {
            let name = gist_file.filename;
            PathBuf::from(name.strip_suffix(".age").unwrap_or(&name).to_string())
        }
    };

    std::fs::write(&output_path, &plaintext)?;
    eprintln!("Decrypted to {}", output_path.display());

    Ok(())
}
