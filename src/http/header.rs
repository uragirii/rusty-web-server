use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug)]

pub struct Header <'buf>{
  header : HashMap<String, &'buf str>,
}

impl <'buf>TryFrom<&'buf str> for Header<'buf> {
  type Error = String;
  fn try_from(s: &'buf str) -> Result<Self, Self::Error>{
    let mut header:HashMap<String, &'buf str>= HashMap::new();
    let mut header_str = s;
    loop {
      match split_string(header_str) {
          Some((header_line, other_headers)) => {
            let (key, value) = parse_header_line(header_line).ok_or("Invalid Header Line")?;
            // I want to use &str here, instead of String. But to_lowercase would have to take a &mut ref
            // Which would be wrong and would require this trait to get a &mut.
            // Anyways it should not cause that much problems as header keys are small and welp
            // Should be easy to store them on heap
            // DONT USE THIS IS PROD i hope ! 
            header.insert(key.trim().to_lowercase(), value.trim());
            header_str = other_headers;
          },
          None => break
      }
    }
    Ok(Self{
      header
    })
  }
}

// idk lol how to share functions from different modules

fn split_string<'a>(s:&'a str)-> Option<(&'a str, &'a str)> {
  for (i, ch) in s.chars().enumerate() {
    if ch =='\n' {
      return Some((&s[..i], &s[i+1..]));
    }
  }
  None
}

fn parse_header_line(s:&str) -> Option<(&str, &str)> {
  if let Some(i) = s.find(':') {
    // TODO: make it lowercase 
    let key = &s[..i];
    let value = &s[i+1..];
    return Some((key, value));
  }
  None
}