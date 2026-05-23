use clap::Args;

#[derive(Args)]
pub struct Arguments {
    /// Commit messages
    #[arg(short, long)]
    pub message: Vec<String>,

    /// Amend all new modifications to latest
    #[arg(long)]
    pub amend: bool,

    /// Force command to execute
    #[arg(short, long)]
    pub force: bool,

    /// Ignore staged files and stage all
    #[arg(short, long)]
    pub all: bool,
}
