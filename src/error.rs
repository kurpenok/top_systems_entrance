use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum DrawError {
    IoError(io::Error),
    OutOfBounds(String),
}

impl Display for DrawError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrawError::IoError(e) => write!(f, "IO Error Occured: {e}"),
            DrawError::OutOfBounds(msg) => write!(f, "Out of Bounds: {msg}"),
        }
    }
}

impl Error for DrawError {}

impl From<io::Error> for DrawError {
    fn from(value: io::Error) -> Self {
        DrawError::IoError(value)
    }
}
