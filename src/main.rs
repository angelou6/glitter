mod cli;
mod commands;
mod git;
mod publish;
mod stage;
mod subcommands;
mod url;

use crate::{
    cli::Commands,
    subcommands::{publish::Publish, undo::Undo},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::parse();
    match cli.command {
        Commands::Init(args) => {
            git::init(args.message, args.branch);

            match args.origin {
                Some(origin) => {
                    git::setup_origin(&origin);
                    if let Some(Publish::Publish(p_args)) = args.publish_command {
                        if !p_args.is_empty() {
                            println!("Arguments for publish ignored");
                        }
                        git::push_to_main();
                    }
                }
                None => {
                    if let Some(Publish::Publish(p_args)) = args.publish_command {
                        if !p_args.is_empty() {
                            let name = p_args.name.unwrap();
                            let desc = p_args.desc.unwrap_or_default();
                            publish::github(name, desc, p_args.private)
                        } else {
                            let (name, desc, private) = publish::draw().unwrap_or_else(|e| {
                                eprint!("Error: {e}");
                                std::process::exit(1);
                            });
                            publish::github(name, desc, private);
                        }
                    }
                }
            }
        }
        Commands::Push(args) => match args.undo_command {
            Some(Undo::Undo(undo_args)) => {
                git::undo_push(args.force, undo_args.hard, undo_args.commit)
            }
            None if args.amend => git::amend_push(args.message, args.force),
            None => git::push(args.message, args.force, args.all),
        },
        Commands::Commit(args) => match args.undo_command {
            Some(Undo::Undo(undo_args)) => git::undo_commit(undo_args.hard, undo_args.commit),
            None if args.amend => git::amend_commit(args.message),
            None => git::add_and_commit(args.message, args.force, args.all),
        },
        Commands::Add(args) => {
            if args.files.len() > 0 {
                if args.revert {
                    git::revert_stage(args.files);
                } else {
                    git::stage_files(args.files);
                }
            } else {
                stage::draw().unwrap_or_else(|e| {
                    eprint!("Error: {e}");
                    std::process::exit(1);
                });
            }
        }
        Commands::Pull(args) => {
            if args.force {
                git::force_pull(args.yes)
            } else {
                git::pull();
            }
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
