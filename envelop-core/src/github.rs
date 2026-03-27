const BASE_URL: &str = "https://github.com";

pub fn fetch_public_keys(username: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let public_keys_url = format!("{BASE_URL}/{username}.keys");
    let text = reqwest::blocking::get(public_keys_url)?.text()?;
    let public_keys: Vec<String> = text.lines().map(|s| s.to_string()).collect();

    Ok(public_keys)
}
