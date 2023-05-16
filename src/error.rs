pub use crate::class::ClassError;
use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum TMCError {
    /// An error occurred in a generic USB operation
    #[error("USB Error: {source}")]
    Rusb {
        #[from]
        source: rusb::Error,
    },

    /// An error occurred in the handling of a USB TMC class operation
    #[error("USBTMC Error: {source}")]
    Class {
        #[from]
        source: ClassError,
    },

    /// The application requested a string response, but the data from the device was not valid UTF-8
    #[error("Error decoding UTF-8 data: {source}")]
    FromUtf8Error {
        #[from]
        source: FromUtf8Error,
    },
}

pub type TMCResult<T> = Result<T, TMCError>;
