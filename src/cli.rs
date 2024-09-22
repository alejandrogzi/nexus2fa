use clap::{self, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(
    name = "nexus2fa",
    version = env!("CARGO_PKG_VERSION"),
    author = "Alejandro Gonzales-Irribarren <alejandrxgzi@gmail.com>",
    about = "converts .nex to .fa plain simple"
)]
pub struct Args {
    #[clap(
        short = 'n',
        long = "nexus",
        help = "Path to NEXUS file",
        value_name = "NEXUS",
        required = true
    )]
    pub nexus: PathBuf,

    #[clap(
        short = 'f',
        long = "fasta",
        help = "Path to FASTA file",
        value_name = "FASTA",
        required = true
    )]
    pub fasta: PathBuf,
}
