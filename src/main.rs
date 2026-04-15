mod commands;
mod url;

use clap::{Parser, Subcommand, Args};

use crate::{commands::{add_and_commit, amend_commit, force_pull, push, push_as_last}, url::{get_commit_url, get_project_url, open}};

#[derive(Parser)]
#[command(
    name = "glitter",
    about = "Usage: glitter <command> [arguments]",
    subcommand_required = true,
    arg_required_else_help = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Push changes
    Push(PushArgs),
    /// Commit changes
    Commit(CommitArgs),
    /// Force pull and reset local changes
    Pull(PullArgs),
    /// Open the project in the default web browser
    Open(OpenArgs),
}

#[derive(Args)]
struct CommitArgs {
    /// Commit message
    #[arg(short, long)]
    message: Option<String>,

    /// Amend all new modifications to the latest commit
    #[arg(long)]
    amend: bool,

    /// Force commit even without message
    #[arg(short, long)]
    force: bool,
}

#[derive(Args)]
struct PushArgs {
    /// Amend all new modifications to the latest push
    #[arg(long)]
    amend: bool,

    /// Force push
    #[arg(short, long)]
    force: bool,

    /// Commit message
    #[arg(short, long)]
    message: Option<String>,
}

#[derive(Args)]
struct PullArgs {
    /// Skip warning
    #[arg(short, long)]
    yes: bool,
}

#[derive(Args)]
struct OpenArgs {
    /// Open a specific commit
    commit: Option<String>,

    /// Print the URL instead of opening it
    #[arg(long)]
    dump: bool,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Push(args) => {
            if args.amend {
                push_as_last(
                    args.message.as_deref().unwrap_or(""),
                    args.force,
                );
            } else {
                push(
                    args.message.as_deref().unwrap_or(""),
                    args.force,
                );
            }
        }
        Commands::Commit(args) => {
            if args.amend {
                amend_commit(&args.message.unwrap_or("".to_owned()));
            } else {
                add_and_commit(&args.message.unwrap_or("".to_owned()), args.force);
            }
        }
        Commands::Pull(args) => {
            force_pull(args.yes);
        }
        Commands::Open(args) => {
            let remote = get_project_url();
            match args.commit {
                Some(commit) => {
                    let url = get_commit_url(&commit);
                    if !args.dump {
                        open(&url);
                    } else {
                        println!("{url}")
                    }
                }
                None => {
                    if !args.dump {
                        open(&remote);
                    } else {
                        println!("{remote}")
                    }
                }
            }
        }
    }
}
