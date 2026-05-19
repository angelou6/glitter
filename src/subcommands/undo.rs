use clap::{Args, ValueEnum};

#[derive(Clone, ValueEnum)]
pub enum UndoTarget {
    Commit,
    Push,
}

#[derive(Args)]
pub struct Arguments {
    /// Target of the undo
    pub action: UndoTarget,

    /// Undo hard
    #[arg(long)]
    pub hard: bool,

    /// Revert to this commit
    #[clap(default_value = "HEAD~")]
    pub commit: String,
}
