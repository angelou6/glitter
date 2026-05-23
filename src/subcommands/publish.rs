use clap::Args;

#[derive(Args)]
pub struct Arguments {
    /// Name of repository
    #[arg(short, long)]
    pub name: Option<String>,

    /// Description of repository
    #[arg(short, long)]
    pub desc: Option<String>,

    /// Set repository visibility to private
    #[arg(short, long)]
    pub private: bool,

    /// Push to origin instead of using github-cli
    #[arg(short, long)]
    pub origin: Option<String>,
}

impl Arguments {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.desc.is_none() && !self.private
    }
}
