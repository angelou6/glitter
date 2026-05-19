use crate::subcommands;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "glitter",
    about = "Opinionated git shortcuts",
    subcommand_required = true,
    arg_required_else_help = true
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a git repo
    Init(subcommands::init::Arguments),

    /// Publish to github or origin
    Publish(subcommands::publish::Arguments),

    /// Stage all files and commit
    Commit(subcommands::commit_push::Arguments),

    /// Stage, commit, and push changes
    Push(subcommands::commit_push::Arguments),

    /// Undo an action
    Undo(subcommands::undo::Arguments),

    /// Stage files
    Add(subcommands::small_commands::AddArgs),

    /// Pull and reset local changes
    Pull(subcommands::small_commands::PullArgs),

    /// Open the repository in the default web browser
    Open(subcommands::small_commands::OpenArgs),
}

pub fn parse() -> Cli {
    Cli::parse()
}
