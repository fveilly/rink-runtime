use std::error;
use std::fmt::{self, Debug, Display};
use std::io;
use std::result;

use serde_json;

#[doc(hidden)]
#[derive(Debug)]
pub enum InkErrorCode {
    Message(String),
    Io(io::Error),
    Json(serde_json::Error),
}

#[derive(Debug)]
pub struct InkError {
    code: InkErrorCode
}

impl InkError {
    fn new(code: InkErrorCode) -> Self {
        InkError {
            code: code
        }
    }
}

impl From<serde_json::Error> for InkError {
     fn from(err: serde_json::Error) -> InkError {
         use serde_json::error::Category;
         match err.classify() {
             Category::Io => {
                 InkError::new(InkErrorCode::Io(err.into()))
             }
             Category::Syntax | Category::Data | Category::Eof => {
                 InkError::new(InkErrorCode::Json(err))
             }
         }
     }
}

impl Display for InkErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InkErrorCode::Message(ref msg) => f.write_str(msg),
            InkErrorCode::Io(ref err) => Display::fmt(err, f),
            InkErrorCode::Json(ref err) => Display::fmt(err, f),
        }
    }
}

impl error::Error for InkError {
    fn description(&self) -> &str {
        match self.code {
            InkErrorCode::Io(ref err) => error::Error::description(err),
            InkErrorCode::Json(ref err) => serde_json::Error::description(err),
            _ => {
                // If you want a better message, use Display::fmt or to_string().
                "Ink error"
            }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.code {
            InkErrorCode::Io(ref err) => Some(err),
            _ => None,
        }
    }
}

impl Display for InkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}
