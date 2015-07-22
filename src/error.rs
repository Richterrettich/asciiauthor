use  std::error::Error;

pub struct BookError {
  message: &'static str
}

impl Error for BookError {
  fn description(&self) -> &str {
    self.message
  }

  fn cause(&self) -> Option<&Error> {
    return None;
  }

}
