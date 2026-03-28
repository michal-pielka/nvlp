use std::path::PathBuf;

use envelop_core::{archive, crypto, github};

pub fn handle(
    paths: &[PathBuf],
    to: &str,
    description: Option<&str>,
    comment: Option<&str>,
    token: Option<&str>,
) -> anyhow::Result<()> {
    let public_keys = github::fetch_public_keys(to)?;
    let public_keys: Vec<&str> = public_keys.iter().map(|k| k.as_str()).collect();

    let payload = archive::pack_files(paths)?;

    let ciphertext_bytes = crypto::encrypt(&payload, &public_keys)?;
    let ciphertext = std::str::from_utf8(&ciphertext_bytes)?;

    let token = github::resolve_token(token)?;

    let gist = github::create_gist(ciphertext, to, description, &token)?;

    github::comment_on_gist(&gist, to, comment, &token)?;

    println!("Secret sent to @{to}\n{}", gist.html_url);

    Ok(())
}
