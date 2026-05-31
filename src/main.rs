mod cli;
mod commands;
mod git_commands;
mod subcommands;
mod tui;

use crate::{
    cli::Commands,
    git_commands::{git, url},
    subcommands::undo,
    tui::{publish, stage},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::parse();
    match cli.command {
        Commands::Init(args) => {
            git::init(args.message, args.branch)?;
        }
        Commands::Publish(args) => match args.origin {
            Some(origin) => {
                eprintln!("Arguments for publish beign ignored");
                git::setup_origin(&origin);
                git::push_to_origin();
            }
            None => {
                if !commands::command_exists("gh") {
                    eprint!("Error: github-cli not found.");
                    std::process::exit(1);
                }

                if !args.is_empty() {
                    let name = args.name.unwrap();
                    let desc = args.desc.unwrap_or_default();
                    publish::github(name, desc, args.private)
                } else {
                    let (name, desc, private) = publish::draw().unwrap_or_else(|e| {
                        eprint!("Error: {e}");
                        std::process::exit(1);
                    });
                    publish::github(name, desc, private);
                }
            }
        },
        Commands::Push(args) => {
            if args.amend {
                git::amend_push(args.message, args.force, args.all)?;
            } else {
                git::push(args.message, args.force, args.all)?;
            }
        }
        Commands::Commit(args) => {
            if args.amend {
                git::amend_commit(args.message, args.all)?;
            } else {
                git::add_and_commit(args.message, args.force, args.all)?;
            }
        }
        Commands::Undo(args) => match args.action {
            undo::UndoTarget::Commit => git::undo_commit(args.hard, args.commit)?,
            undo::UndoTarget::Push => git::undo_push(args.hard, args.commit)?,
        },
        Commands::Add(args) => {
            if args.files.is_empty() && args.revert {
                eprint!("Error: No files to revert");
                std::process::exit(1);
            } else if args.files.is_empty() {
                stage::draw().unwrap_or_else(|e| {
                    eprint!("Error: {e}");
                    std::process::exit(1);
                });
            } else if args.revert {
                git::revert_stage(args.files);
            } else {
                git::stage(args.files);
            }
        }
        Commands::Pull(args) => {
            git::pull(args.force, args.yes);
        }
        Commands::Open(args) => {
            let remote = url::get_project_url();
            match args.commit {
                Some(commit) => {
                    let url = url::get_commit_url(&commit);
                    if !args.dump {
                        url::open(&url);
                    } else {
                        println!("{url}")
                    }
                }
                None if !args.dump => url::open(&remote),
                None => println!("{remote}"),
            }
        }
    }

    Ok(())
}
