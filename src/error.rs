use  std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BookError {
  pub message: &'static str
}

impl Error for BookError {
  fn description(&self) -> &str {
    self.message
  }

  fn cause(&self) -> Option<&Error> {
    return None;
  }
}

impl fmt::Display for BookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
