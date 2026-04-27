mod commands;
mod url;

use std::process;

use clap::{Parser, Subcommand, Args};

use crate::{commands::{add_and_commit, amend_commit, force_pull, push, push_as_last, undo_commit, undo_push}, url::{get_commit_url, get_project_url, open}};

#[derive(Parser)]
#[command(
    name = "glitter",
    about = "Usage: glitter <command> [arguments]",
    version,
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
    message: Vec<String>,

    /// Amend all new modifications to the latest commit
    #[arg(long)]
    amend: bool,

    /// Force commit even without message
    #[arg(short, long)]
    force: bool,

    /// Undo last commit
    #[arg(long)]
    undo: bool,

    /// Undo last push hard
    #[arg(long)]
    hard_undo: bool,
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
    message: Vec<String>,

    /// Undo last push
    #[arg(long)]
    undo: bool,

    /// Undo last push hard
    #[arg(long)]
    hard_undo: bool,
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
            if args.message.len() > 2 {
                eprintln!("You can only use a max of 2 messages.");
                process::exit(1);
            }

            if args.undo || args.hard_undo {
                undo_push(args.force, args.hard_undo);
            } else if args.amend {
                push_as_last(
                    args.message,
                    args.force,
                );
            } else {
                push(
                    args.message,
                    args.force,
                );
            }
        }
        Commands::Commit(args) => {
            if args.message.len() > 2 {
                eprintln!("You can only use a max of 2 messages.");
                process::exit(1);
            }

            if args.undo || args.hard_undo {
                undo_commit(args.hard_undo);
            } else if args.amend {
                amend_commit(args.message);
            } else {
                add_and_commit(
                    args.message,
                    args.force
                );
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
