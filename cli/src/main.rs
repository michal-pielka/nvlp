use clap::Parser;
use nvlp::cli::{Args, Command};
use nvlp::commands;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Send {
            file,
            to,
            description,
            comment,
            token,
        } => commands::send::handle(
            &file,
            &to,
            description.as_deref(),
            comment.as_deref(),
            token.as_deref(),
        ),

        Command::Open {
            url,
            identity,
            output,
            stdout,
            token,
        } => commands::open::handle(
            &url,
            identity.as_deref(),
            output.as_deref(),
            stdout,
            token.as_deref(),
        ),

        Command::Encrypt { file, to, output } => {
            commands::encrypt::handle(file.as_deref(), &to[..], output.as_deref())
        }

        Command::Decrypt {
            file,
            identity,
            output,
        } => commands::decrypt::handle(file.as_deref(), identity.as_deref(), output.as_deref()),

        Command::Keys { username } => commands::keys::handle(&username),
    }
}
