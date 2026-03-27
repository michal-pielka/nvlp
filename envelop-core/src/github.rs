use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;

const BASE_URL: &str = "https://github.com";
const API_URL: &str = "https://api.github.com";

// TODO: error handling
pub fn fetch_public_keys(username: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let public_keys_url = format!("{BASE_URL}/{username}.keys");
    let text = reqwest::blocking::get(public_keys_url)?
        .error_for_status()?
        .text()?;
    let public_keys: Vec<String> = text
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(public_keys)
}

#[derive(Deserialize)]
pub struct Gist {
    id: String,
    html_url: String,
}

// TODO: error handling
pub fn create_gist(
    content: &str,
    recipient: &str,
    token: &str,
) -> Result<Gist, Box<dyn std::error::Error>> {
    let create_gist_url = format!("{API_URL}/gists");

    let client = Client::new();
    let body = json!({
        "description": format!("Envelop for {recipient}"),
        "public": false,
        "files": {"envelop.age": {"content": content}},
    });
    let resp = client
        .post(create_gist_url)
        .json(&body)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {token}"))
        .header("X-GitHub-Api-Version", "2026-03-10")
        .send()?
        .error_for_status()?;

    Ok(resp.json()?)
}
