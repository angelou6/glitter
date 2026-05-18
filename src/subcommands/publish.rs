use clap::{Args, Subcommand};

#[derive(Args)]
pub struct Arguments {
    /// Name of repo
    #[arg(short, long)]
    pub name: Option<String>,

    /// Description of repo
    #[arg(short, long)]
    pub desc: Option<String>,

    /// Repo visibility
    #[arg(short, long)]
    pub private: bool,
}

impl Arguments {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.desc.is_none() && !self.private
    }
}

#[derive(Subcommand)]
pub enum Publish {
    /// Publish online
    Publish(Arguments),
}
