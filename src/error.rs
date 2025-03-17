use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum ServerError {
  #[error("IO error: {0}")]
  Io(io::Error),
}

impl From<io::Error> for ServerError {
  fn from(error: io::Error) -> Self {
    ServerError::Io(error)
  }
}
