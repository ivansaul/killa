use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Network(#[from] reqwest::Error),
}
