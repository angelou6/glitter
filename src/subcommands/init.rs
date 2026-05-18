use crate::subcommands::publish::Publish;
use clap::Args;

#[derive(Args)]
pub struct Arguments {
    /// Commit message
    #[arg(short, long)]
    pub message: Vec<String>,

    /// Declare origin
    #[arg(short, long)]
    pub origin: Option<String>,

    #[arg(short, long)]
    #[clap(default_value = "main")]
    /// Declare branch
    pub branch: String,

    #[command(subcommand)]
    pub publish_command: Option<Publish>,
}
