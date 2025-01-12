pub mod file;
pub mod mode;

mod iter;

use std::{io, num};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("invalid input: {0}")]
    InvalidInput(&'static str),

    #[error("invalid data: {0}")]
    InvalidData(&'static str),

    #[error("failed to parse integer: {0}")]
    ParseIntError(#[from] num::ParseIntError),

    #[error("failed to parse float: {0}")]
    ParseFloatError(#[from] num::ParseFloatError),
}
