use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
pub enum Method {
  GET,
  POST,
  PUT,
  DELETE,
}

impl FromStr for Method {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err>{
    match s {
        "GET" => Ok(Method::GET),
        "POST" => Ok(Method::POST),
        "PUT" => Ok(Method::PUT),
        "DELETE" => Ok(Method::DELETE),
        _ => Err("Unsupported Method".to_string())
    }
  }
}

impl fmt::Display for Method {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
        Method::GET => write!(f,"GET"),
        Method::POST => write!(f,"POSt"),
        Method::PUT => write!(f,"PUT"),
        Method::DELETE => write!(f,"DELETE"),
    }
  }
}