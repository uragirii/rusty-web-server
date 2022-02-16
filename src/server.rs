use std::net::TcpListener;
use std::io::Read;

pub struct Server {
  addr : String,
}


impl Server {
  pub fn new(addr:String) -> Self {
      Self{
          addr
      }
  }
  pub fn listen(&self) {
      let stream = TcpListener::bind(&self.addr).unwrap();
      println!("Server is listening on {}",self.addr);

      loop {
          match stream.accept() {
              Ok((mut stream, _)) => {
                  let mut buffer = [0; 1024];
                  match stream.read(&mut buffer) {
                      Ok(_) => println!("{}", String::from_utf8_lossy(&buffer)),
                      Err(_) => println!("Error reading") 
                  }
              },
              Err(_) => println!("Something went wrong")
          }
      }


  }
}

