#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to parse SSH public key: {0}")]
    KeyParse(String),

    #[error("encryption failed: {0}")]
    Encrypt(String),

    #[error("decryption failed: {0}")]
    Decrypt(String),

    #[error("GitHub API error: {0}")]
    GitHub(#[from] reqwest::Error),

    #[error("no GitHub token found - set GITHUB_TOKEN or run `gh auth login`")]
    NoToken,

    #[error("user {0} has no SSH keys on GitHub")]
    NoKeys(String),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("gist contains no files")]
    EmptyGist,
}

pub type Result<T> = std::result::Result<T, Error>;
