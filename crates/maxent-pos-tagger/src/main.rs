use clap::Parser;

use crate::options::{Args, Command};

mod options;

fn main() {
    let args = Args::parse();

    dbg!(&args);

    match args.cmd {
        Command::Train{
            props,
            model,
            train_file,
            verbose,
            verbose_results
        } => {
            maxent_tagger::run_training()
        }
        _ => todo!()
    }
}
