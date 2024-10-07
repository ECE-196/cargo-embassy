use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct InitArgs {
    #[arg(help = "The name of the Embassy project to create.")]
    pub name: String,
}
