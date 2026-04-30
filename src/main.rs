mod commands;
mod url;

use clap::{Args, Parser, Subcommand};

use crate::{
    commands::{
        add_and_commit, amend_commit, amend_push, force_pull, push, undo_commit, undo_push,
    },
    url::{get_commit_url, get_project_url, open},
};

#[derive(Parser)]
#[command(
    name = "glitter",
    about = "Usage: glitter <command> [arguments]",
    subcommand_required = true,
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Stage, commit, and push changes
    Push(CommitPushArgs),
    /// Stage all files and commit
    Commit(CommitPushArgs),
    ///  Force pull and reset local changes
    Pull(PullArgs),
    /// Open the project in the default web browser
    Open(OpenArgs),
}

#[derive(Subcommand)]
enum UndoCommads {
    /// Undo latest
    Undo(UndoArgs),
}

#[derive(Args)]
struct UndoArgs {
    /// Undo hard
    #[arg(long)]
    hard: bool,
}

#[derive(Args)]
struct CommitPushArgs {
    /// Commit message
    #[arg(short, long)]
    message: Vec<String>,

    /// Amend all new modifications to latest
    #[arg(long)]
    amend: bool,

    /// Force command to execute
    #[arg(short, long)]
    force: bool,

    /// Ignore staged files and stage all
    #[arg(short, long)]
    all: bool,

    #[command(subcommand)]
    undo_command: Option<UndoCommads>,
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

fn validate_messages(messages: &Vec<String>) -> Result<(), String> {
    if messages.len() > 2 {
        Err(String::from("You can only use a max of 2 messages."))
    } else {
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Push(args) => {
            validate_messages(&args.message)?;
            match args.undo_command {
                Some(UndoCommads::Undo(undo_args)) => undo_push(args.force, undo_args.hard),
                None if args.amend => amend_push(args.message, args.force),
                None => push(args.message, args.force, args.all),
            }
        }
        Commands::Commit(args) => {
            validate_messages(&args.message)?;
            match args.undo_command {
                Some(UndoCommads::Undo(undo_args)) => undo_commit(undo_args.hard),
                None if args.amend => amend_commit(args.message),
                None => add_and_commit(args.message, args.force, args.all),
            }
        }
        Commands::Pull(args) => force_pull(args.yes),
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
    Ok(())
}
