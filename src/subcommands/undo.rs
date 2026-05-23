use clap::{Args, ValueEnum};

#[derive(Clone, ValueEnum)]
pub enum UndoTarget {
    Commit,
    Push,
}

#[derive(Args)]
pub struct Arguments {
    /// Target of undo command
    pub action: UndoTarget,

    /// Also undo changes locally
    #[arg(long)]
    pub hard: bool,

    /// Revert to this commit
    #[clap(default_value = "HEAD~")]
    pub commit: String,
}
