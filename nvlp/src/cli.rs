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
    /// Encrypt a file (or stdin) to a GitHub user's SSH keys
    Encrypt {
        /// File to encrypt (reads from stdin if omitted)
        file: Option<PathBuf>,

        /// GitHub username(s) of the recipient(s)
        #[arg(short, long, value_name = "USERNAME", required = true, action = clap::ArgAction::Append)]
        to: Vec<String>,

        /// Output file path (defaults to <filename>.age, or stdout when reading from stdin)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Decrypt a .age file (or stdin) encrypted with nvlp
    Decrypt {
        /// Encrypted file to decrypt (reads from stdin if omitted)
        file: Option<PathBuf>,

        /// Path to SSH private key for decryption
        #[arg(short, long, value_name = "FILE")]
        identity: Option<PathBuf>,

        /// Output file path (defaults to input filename without .age, or stdout when reading from stdin)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Encrypt and send a file to a GitHub user as a private Gist
    Send {
        /// File to encrypt and send
        file: PathBuf,

        /// GitHub username(s) of the recipient(s)
        #[arg(short, long, value_name = "USERNAME", required = true, action = clap::ArgAction::Append)]
        to: Vec<String>,

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

    /// Decrypt a file from a nvlp Gist
    Open {
        /// URL of the Gist to open (e.g. https://gist.github.com/user/abc123)
        url: String,

        /// Path to SSH private key for decryption
        #[arg(short, long, value_name = "FILE")]
        identity: Option<PathBuf>,

        /// Output file path (defaults to original filename)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Print decrypted content to stdout instead of saving to a file
        #[arg(long)]
        stdout: bool,

        /// GitHub personal access token (falls back to GITHUB_TOKEN or `gh auth token` output)
        #[arg(long, value_name = "TOKEN")]
        token: Option<String>,
    },

    /// List a GitHub user's SSH public keys
    Keys {
        /// GitHub username to look up
        username: String,
    },
}
