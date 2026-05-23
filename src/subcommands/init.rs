use clap::Args;

#[derive(Args)]
pub struct Arguments {
    /// Commit messages
    #[arg(short, long)]
    pub message: Vec<String>,

    #[arg(short, long)]
    #[clap(default_value = "main")]
    /// Declare branch
    pub branch: String,
}
