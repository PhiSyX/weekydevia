#[derive(Debug, thiserror::Error)]
#[error("weekydevia error: {0}")]
pub enum Error {
    /// Erreur liée à l'I/O.
    IO(#[from] std::io::Error),
}
