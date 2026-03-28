use std::path::PathBuf;

use envelop_core::{archive, crypto, github};

use super::fetch_all_keys;

pub fn handle(
    paths: &[PathBuf],
    to: &[String],
    description: Option<&str>,
    comment: Option<&str>,
    token: Option<&str>,
) -> anyhow::Result<()> {
    let public_keys = fetch_all_keys(to)?;
    let public_keys: Vec<&str> = public_keys.iter().map(|k| k.as_str()).collect();

    let payload = archive::pack_files(paths)?;

    let ciphertext_bytes = crypto::encrypt(&payload, &public_keys)?;
    let ciphertext = std::str::from_utf8(&ciphertext_bytes)?;

    let token = github::resolve_token(token)?;

    let recipients = to.join(", @");
    let default_description = format!("Envelop for @{recipients}");
    let description = description.unwrap_or(&default_description);

    let gist = github::create_gist(ciphertext, &recipients, Some(description), &token)?;

    for recipient in to {
        github::comment_on_gist(&gist, recipient, comment, &token)?;
    }

    println!("Secret sent to @{recipients}\n{}", gist.html_url);

    Ok(())
}
