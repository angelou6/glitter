mod commands;
mod git;
mod publish;
mod stage;
mod url;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "glitter",
    about = "Opinionated git shortcuts",
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
    /// Initialize a git repo
    Init(InitArgs),
    /// Stage all files and commit
    Commit(CommitPushArgs),
    /// Stage files
    Add(AddArgs),
    ///  Force pull and reset local changes
    Pull(PullArgs),
    /// Open the repository in the default web browser
    Open(OpenArgs),
}

#[derive(Subcommand)]
enum UndoCommads {
    /// Undo latest
    Undo(UndoArgs),
}

#[derive(Subcommand)]
enum PublishCommand {
    /// Publish online, public by default
    Publish(PublishArgs),
}

#[derive(Args)]
struct InitArgs {
    /// Commit message
    #[arg(short, long)]
    message: Vec<String>,

    /// Force commit to execute
    #[arg(short, long)]
    force: bool,

    #[command(subcommand)]
    publish_command: Option<PublishCommand>,
}

#[derive(Args)]
struct PublishArgs {
    /// Name of repo
    #[arg(short, long)]
    name: Option<String>,

    /// Description of repo
    #[arg(short, long)]
    desc: Option<String>,

    /// Repo visibility
    #[arg(short, long)]
    private: bool,
}

impl PublishArgs {
    fn is_empty(&self) -> bool {
        self.name.is_none() && self.desc.is_none() && !self.private
    }
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
struct AddArgs {
    files: Vec<String>,

    /// Revert
    #[arg(short, long)]
    revert: bool,
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
    #[arg(short, long)]
    dump: bool,
}

fn validate_messages(messages: &Vec<String>) -> Result<(), String> {
    if messages.len() > 2 {
        Err(String::from("You can only have a name and description."))
    } else {
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init(args) => {
            validate_messages(&args.message)?;
            git::init(args.message);
            if let Some(PublishCommand::Publish(p_args)) = args.publish_command {
                if !p_args.is_empty() {
                    let name = p_args.name.unwrap();
                    let desc = p_args.desc.unwrap_or(String::new());
                    publish::github(name, desc, p_args.private)
                } else if let Some((name, desc, private)) = publish::draw() {
                    publish::github(name, desc, private)
                }
            }
        }
        Commands::Push(args) => {
            validate_messages(&args.message)?;
            match args.undo_command {
                Some(UndoCommads::Undo(undo_args)) => git::undo_push(args.force, undo_args.hard),
                None if args.amend => git::amend_push(args.message, args.force),
                None => git::push(args.message, args.force, args.all),
            }
        }
        Commands::Commit(args) => {
            validate_messages(&args.message)?;
            match args.undo_command {
                Some(UndoCommads::Undo(undo_args)) => git::undo_commit(undo_args.hard),
                None if args.amend => git::amend_commit(args.message),
                None => git::add_and_commit(args.message, args.force, args.all),
            }
        }
        Commands::Add(args) => {
            if args.files.len() > 0 {
                if args.revert {
                    git::revert_stage(args.files);
                } else {
                    git::stage_files(args.files);
                }
            } else {
                let mut interface = stage::Interface::new()?;
                interface.draw()?;
            }
        }
        Commands::Pull(args) => git::force_pull(args.yes),
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
