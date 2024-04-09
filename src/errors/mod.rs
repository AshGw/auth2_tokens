use base64::DecodeError as DecErr;
use std::fmt::{self, Display, Formatter};
use crate::consts::{CV_MAX_SIZE,CV_MIN_SIZE}; 

#[derive(Debug)]
pub struct TokenError;

#[derive(Debug)]
pub struct NonceError;

#[derive(Debug)]
pub struct StateError;

#[derive(Debug)]
pub enum CodeVerfierError {
    TooBig,
    TooSmall,
}

pub type DecodeError = DecErr;

#[derive(Debug)]
pub enum B64Error {
    InvalidEncoding,
    DecodeError,
}

impl Display for B64Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidEncoding => write!(fmt, "Invalid Base64 encoding."),
            Self::DecodeError => write!(fmt, "Cannot decode the given value"),
        }
    }
}

impl Display for CodeVerfierError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Self::TooBig => write!(fmt, "it must be less than {}",CV_MAX_SIZE), 
            Self::TooSmall => write!(fmt, "it must be greater than {}", CV_MIN_SIZE), 
        }
    }
}
