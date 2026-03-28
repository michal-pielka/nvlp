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

#[derive(Debug, Deserialize)]
pub struct Gist {
    pub id: String,
    pub html_url: String,
}

// TODO: error handling
pub fn create_gist(
    content: &str,
    recipient: &str,
    description: Option<&str>,
    token: &str,
) -> Result<Gist, Box<dyn std::error::Error>> {
    let create_gist_url = format!("{API_URL}/gists");

    let description = match description {
        Some(c) => c.to_string(),
        None => format!("Envelop for {recipient}"),
    };

    let client = Client::new();
    let body = json!({
        "description": description,
        "public": false,
        "files": {"envelop.age": {"content": content}},
    });
    let resp = client
        .post(create_gist_url)
        .json(&body)
        .header("User-Agent", "envelop")
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {token}"))
        .header("X-GitHub-Api-Version", "2026-03-10")
        .send()?
        .error_for_status()?;

    Ok(resp.json()?)
}

pub fn comment_on_gist(
    gist: &Gist,
    recipient: &str,
    comment: Option<&str>,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let create_gist_url = format!("{API_URL}/gists/{}/comments", gist.id);

    let comment = match comment {
        Some(c) => c.to_string(),
        None => format!(
            "@{recipient} you have a new envelop!\n```bash\nenvelop open {}\n```",
            gist.html_url
        ),
    };

    let client = Client::new();
    let body = json!({
        "body": comment,
    });
    let _resp = client
        .post(create_gist_url)
        .json(&body)
        .header("User-Agent", "envelop")
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {token}"))
        .header("X-GitHub-Api-Version", "2026-03-10")
        .send()?
        .error_for_status()?;

    Ok(())
}

// TODO: error handling
pub fn resolve_token(explicit_token: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    // Prioritize --token flag
    if let Some(token) = explicit_token {
        return Ok(token.to_string());
    }

    // If --token not provided, check GITHUB_TOKEN env variable
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        return Ok(token);
    }

    // If neither --token nor GITHUB_TOKEN provided, fallback to github cli: "gh auth token"
    let output = std::process::Command::new("gh")
        .args(["auth", "token"])
        .output()?;
    let stdout = output.stdout;

    Ok(String::from_utf8(stdout)?.trim().to_string())
}
