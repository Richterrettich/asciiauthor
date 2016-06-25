
use std::error::Error;
use std::fmt;
use std::convert::From;
use std::io;

#[derive(Debug)]
pub enum BookError {
   IoBookError(io::Error),
   NormalBookError(String),
}

impl Error for BookError {
  fn description(&self) -> &str {
    match *self {
      BookError::IoBookError(ref err) => err.description(),
      BookError::NormalBookError(ref desc) => desc
    }
  }

  fn cause(&self) -> Option<&Error> {
    return None;
  }
}

impl fmt::Display for BookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
        BookError::IoBookError(ref err) => write!(f, "{}", err.description()),
        BookError::NormalBookError(ref desc) => write!(f, "{}", desc)
      }
    }
}

impl From<io::Error> for BookError {
  fn from(err: io::Error) ->  BookError {
    BookError::IoBookError(err)
  }
}


