use nvlp_core::github;

pub fn handle(username: &str) -> anyhow::Result<()> {
    let public_keys = github::fetch_public_keys(username)?;

    for key in &public_keys {
        println!("{key}");
    }

    Ok(())
}
