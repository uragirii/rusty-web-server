use std::net::TcpListener;
use std::io::Read;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.listen();
}


struct Server {
    addr : String,
}


impl Server {
    fn new(addr:String) -> Self {
        Self{
            addr
        }
    }
    fn listen(&self) {
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

