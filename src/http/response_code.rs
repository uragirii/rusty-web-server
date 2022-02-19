use std::fmt;

#[derive(Debug,Clone, Copy)]
pub enum ResponseCodes {
  Ok = 200, 
  BadRequest = 400,
  // The classical 404
  NotFound = 404,
  ServerError = 500
}

impl ResponseCodes { 
  pub fn reason_phrase(&self) -> &str {
    match self {
        ResponseCodes::Ok => "Ok",
        ResponseCodes::BadRequest => "Bad Request",
        ResponseCodes::NotFound => "Not Found",
        ResponseCodes::ServerError => "Internal Server Error",
    }
  }
}

impl fmt::Display for ResponseCodes {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {}", *self as u16, self.reason_phrase())
  }
}
