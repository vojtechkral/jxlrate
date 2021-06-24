use std::io;
use std::result;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid file signature")]
    BadSignature,
    #[error("Unexpected EOF")]
    UnexpectedEof,
    #[error("I/O Error")]
    Io(#[from] io::Error),
}

pub type Result<T, E = Error> = result::Result<T, E>;
