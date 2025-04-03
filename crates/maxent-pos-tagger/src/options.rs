use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Generate a property file
    GenProps,
    /// Train a model
    Train {
        /// Property file for this model
        #[arg(long)]
        props: Option<PathBuf>,
        /// Model file
        #[arg(long)]
        model: Option<PathBuf>,
        /// Training data
        #[arg(long)]
        train_file: Option<PathBuf>,
        /// 
        #[arg(short, long)]
        verbose: bool,
        ///
        #[arg(long, default_value = "true")]
        verbose_results: bool,
    },
    /// Tag text
    Tag,
    /// Test model
    Test,
    /// Dump model
    Dump,
}
