use std::path;

use clap::Parser;

#[derive(clap::Parser)]
pub struct Args {
    /// Répertoire template README
    #[clap(short = 'd')]
    pub template_directory: path::PathBuf,
    /// Fichier de sortie
    #[clap(short = 'o')]
    pub output_directory: path::PathBuf,
}

pub fn arguments() -> Args {
    Args::parse()
}
