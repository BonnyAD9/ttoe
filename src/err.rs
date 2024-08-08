use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't place there. The place is already populated.")]
    AlreadyPopulated,
    #[error(transparent)]
    Termal(#[from] termal::error::Error),
}
