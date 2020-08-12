use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("crossterm error")]
	Disconnect(#[from] crossterm::ErrorKind),
	#[error("io error")]
	IO(#[from] std::io::Error),
	#[error("unknown error")]
	Unknown,
}
