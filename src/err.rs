use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't place there. The place is already populated.")]
    AlreadyPopulated,
    #[error("Rage quit :)")]
    RageQuit,
    #[error(transparent)]
    Termal(#[from] termal::error::Error),
    #[error(transparent)]
    Pareg(#[from] pareg::ArgError),
}
