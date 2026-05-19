use clap::Args;

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

    /// Push to origin instead
    #[arg(short, long)]
    pub origin: Option<String>,
}

impl Arguments {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.desc.is_none() && !self.private
    }
}
