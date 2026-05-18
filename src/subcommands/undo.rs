use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Undo {
    /// Undo latest
    Undo(Arguments),
}

#[derive(Args)]
pub struct Arguments {
    /// Undo hard
    #[arg(long)]
    pub hard: bool,

    // Revert to this commit
    #[clap(default_value = "HEAD~")]
    pub commit: String,
}
