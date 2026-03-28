use clap::Parser;
use envelop_cli::cli::{Args, Command};
use envelop_cli::commands;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Send {
            files,
            to,
            description,
            comment,
            token,
        } => commands::send::handle(
            &files,
            &to,
            description.as_deref(),
            comment.as_deref(),
            token.as_deref(),
        ),

        Command::Open {
            url,
            identity,
            output,
        } => commands::open::handle(&url, identity.as_deref(), &output),

        Command::Encrypt { files, to, output } => {
            commands::encrypt::handle(&files, &to, output.as_deref())
        }

        Command::Decrypt {
            file,
            identity,
            output,
        } => commands::decrypt::handle(&file, identity.as_deref(), &output),

        Command::Keys { username } => commands::keys::handle(&username),
    }
}
