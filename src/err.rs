use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't place there. The place is already populated.")]
    AlreadyPopulated,
    #[error("Cant't place at the given position. It is outside of bounds.")]
    OutOfBounds,
    #[error(transparent)]
    Termal(#[from] termal::error::Error),
}
