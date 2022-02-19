use super::response_code::ResponseCodes;
use super::header::Header;
use std::fmt;


#[derive(Debug)]
pub struct Response<'a> {
  code : ResponseCodes,
  headers : Header<'a>,
  body:&'a str
}

impl<'a> Response<'a> {
  pub fn new(code : ResponseCodes, headers:Header<'a>, body:&'a str) -> Self {
    Self {
      code, headers, body
    }
  }
}

impl<'a> fmt::Display for Response<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "HTTP/1.1 {}{}\n\n{}",self.code, self.headers, self.body);
    Ok(())
  }
}