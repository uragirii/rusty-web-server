use super::header::Header;
use super::method::Method;
use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt;
use std::str;

#[derive(Debug)]
pub struct Request <'buf>{
  method: Method,
  path: &'buf str,
  query_string: Option<&'buf str>,
  headers: Header<'buf>
}

impl <'buf> TryFrom<&'buf[u8]> for Request<'buf>{
  type  Error = ParseError;
  // GET / HTTP/1.1
  fn try_from(request:&'buf [u8]) -> Result<Self, Self::Error> {

    let request = str::from_utf8(request).or(Err(ParseError::InvalidRequest))?;


    if let Some(i) = request.find('\n') {
      let (request, headers) = (&request[..i], &request[i+1..]);
      let headers_end_index = headers.find("\r\n\r\n");


      // Headers should always end with \r\n\r\n
      if headers_end_index.is_none() {
        return Err(ParseError::InvalidRequest);
      }
      let headers = &headers[..headers_end_index.unwrap()];


      let headers = Header::try_from(headers);


      //TODO: Is this the best way to handle? how about `ok_or` or something like that?
      if headers.is_err() {
        return Err(ParseError::InvalidRequest);
      }
      // Now its safe to unwrap;
      let headers = headers.unwrap();


      let (method, request) = split_string(request).ok_or(ParseError::InvalidRequest)?;

      let (mut path, protocol) = split_string(request).ok_or(ParseError::InvalidRequest)?;

      if protocol.trim() != "HTTP/1.1" {
        return Err(ParseError::UnsupportedProtocol);
      }

      let method =  Method::from_str(method).or(Err(ParseError::InvalidMethod))?;

      let mut query_string  = None;

      if let Some(i) = path.find("?") {
        query_string = Some(&path[i+1..]);
        path = &path[..i];
      }
    
    return Ok(Self{
      method,
      path,
      query_string,
      headers
    })
    }
    Err(ParseError::InvalidRequest)    
  }
}

impl<'buf> fmt::Display for Request<'buf> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   write!(f, "[{}] {}",self.method, self.path)
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
  InvalidRequest,
  UnsupportedProtocol
}