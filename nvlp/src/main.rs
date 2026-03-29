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
            commands::encrypt::handle(&file, &to[..], output.as_deref())
        }

        Command::Decrypt {
            file,
            identity,
            output,
            stdout,
        } => commands::decrypt::handle(&file, identity.as_deref(), output.as_deref(), stdout),

        Command::Keys { username } => commands::keys::handle(&username),
    }
}
