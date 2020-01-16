use std::error;
use std::fmt;
use serde::{Deserialize, Serialize};
use super::*;

pub type SPResult<T> = std::result::Result<T, SPError>;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum SPError {
    No(String),
    Undefined,
}

impl fmt::Display for SPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SPError::Undefined => write!(f, "An undefined SP error!"),
            SPError::No(s) => write!(f, "Oh No: {}", s),
        }
    }
}

impl error::Error for SPError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}