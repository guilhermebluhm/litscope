use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError{
    Config(String)
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self { 
            AppError::Config(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self { 
            AppError::Config(_) => None
        }
    }
}