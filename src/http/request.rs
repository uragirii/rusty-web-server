use super::method::Method;
use std::convert::TryFrom;
use std::str::FromStr;

use std::str;

#[derive(Debug)]
pub struct Request <'buf>{
  method: Method,
  path: &'buf str,
  query_string: Option<&'buf str>,
}

impl <'buf> TryFrom<&'buf[u8]> for Request<'buf>{
  type  Error = ParseError;
  // GET / HTTP/1.1
  fn try_from(request:&'buf [u8]) -> Result<Self, Self::Error> {
    let request = str::from_utf8(request).or(Err(ParseError::InvalidRequest))?;
    let (method, request) = split_string(request).ok_or(ParseError::InvalidRequest)?;
    let (mut path, request) = split_string(request).ok_or(ParseError::InvalidRequest)?;
    let (protocol, _) = split_string(request).ok_or(ParseError::InvalidRequest)?;

    if protocol != "HTTP/1.1"  {
      return Err(ParseError::InvalidProtocol)
    } 

    let method =  Method::from_str(method).or(Err(ParseError::InvalidMethod))?;

    let mut query_string  = None;

    if let Some(i) = path.find("?") {
      query_string = Some(&path[i+1..]);
      path = &path[..i];
    }
    
    Ok(Self{
      method,
      path,
      query_string,
    })
  }
}

fn split_string(request:&str) -> Option<(&str, &str)> {
  for (i, ch) in request.chars().enumerate() {
    if ch ==' ' {
      return Some((&request[..i], &request[i+1..]));
    }
  }
  None
}

#[derive(Debug)]
pub enum ParseError { 
  InvalidEncoding ,
  InvalidMethod,
  InvalidPath,
  InvalidProtocol,
  InvalidRequest
}