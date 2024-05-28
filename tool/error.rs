#[derive(Debug, thiserror::Error)]
#[error("weekydevia error: {0}")]
pub enum Error {
    IO(#[from] std::io::Error),
    Feed(#[from] rss::Error),
}
