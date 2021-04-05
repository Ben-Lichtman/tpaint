use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("IO error")]
	IOError(#[from] std::io::Error),
	#[error("crossterm error")]
	Disconnect(#[from] crossterm::ErrorKind),
}
