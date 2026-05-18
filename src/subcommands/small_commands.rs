/// Small commands that don't deserve their own file
use clap::Args;

#[derive(Args)]
pub struct AddArgs {
    /// Files to be staged
    pub files: Vec<String>,

    /// Revert
    #[arg(short, long)]
    pub revert: bool,
}

#[derive(Args)]
pub struct PullArgs {
    /// Force pull
    #[arg(short, long)]
    pub force: bool,

    /// Skip warning
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Args)]
pub struct OpenArgs {
    /// Open a specific commit
    pub commit: Option<String>,

    /// Print the URL instead of opening it
    #[arg(short, long)]
    pub dump: bool,
}
