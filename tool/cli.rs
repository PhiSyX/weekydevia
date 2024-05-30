use std::path;

use clap::Parser;

#[derive(clap::Parser)]
pub struct Args {
    /// Répertoire draft, avec le fichier template README.md
    #[clap(short = 'd')]
    pub draft_directory: path::PathBuf,
    /// Fichier de sortie
    #[clap(short = 'o')]
    pub output_directory: path::PathBuf,
    /// Supprime le répertoire draft
    #[clap(default_value = "false", long)]
    pub delete: bool
}

pub fn arguments() -> Args {
    Args::parse()
}
