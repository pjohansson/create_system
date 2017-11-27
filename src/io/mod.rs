//! Modules for reading coordinate files.

use std::error::Error;
use std::io;
use std::num::{ParseFloatError, ParseIntError};

pub mod gromos;

#[derive(Debug)]
/// Errors when reading files.
pub enum GrafenIoError {
    /// Something went wrong when parsing the file.
    ParseError(String),
    /// Not all required data was found in the file.
    EOF(String),
}
use self::GrafenIoError::*;

impl From<io::Error> for GrafenIoError {
    fn from(_: io::Error) -> Self {
        // TODO: Better diagnostics here
        EOF("Could not read file".into())
    }
}

impl From<ParseFloatError> for GrafenIoError {
    fn from(err: ParseFloatError) -> Self {
        EOF(err.description().into())
    }
}

impl From<ParseIntError> for GrafenIoError {
    fn from(err: ParseIntError) -> Self {
        EOF(err.description().into())
    }
}