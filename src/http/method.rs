use std::str::FromStr;

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