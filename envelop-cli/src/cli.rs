use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about = "Send encrypted files to GitHub users via their SSH keys"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Encrypt and send files to a GitHub user as a private Gist
    Send {
        /// Files to encrypt and send
        #[arg(required = true, num_args = 1..)]
        files: Vec<PathBuf>,

        /// GitHub username of the recipient
        #[arg(short, long, value_name = "USERNAME")]
        to: String,

        /// Custom Gist description
        #[arg(short, long, value_name = "TEXT")]
        description: Option<String>,

        /// Custom comment on the Gist
        #[arg(short, long, value_name = "TEXT")]
        comment: Option<String>,

        /// GitHub personal access token (falls back to GITHUB_TOKEN or `gh auth token` output)
        #[arg(long, value_name = "TOKEN")]
        token: Option<String>,
    },

    /// Decrypt and extract files from an envelop Gist
    Open {
        /// URL of the Gist to open (e.g. https://gist.github.com/user/abc123)
        url: String,

        /// Path to SSH private key for decryption
        #[arg(short, long, value_name = "FILE")]
        identity: Option<PathBuf>,

        /// Directory to extract files into
        #[arg(short, long, value_name = "DIR", default_value = ".")]
        output: PathBuf,
    },

    /// List a GitHub user's SSH public keys
    Keys {
        /// GitHub username to look up
        username: String,
    },
}
