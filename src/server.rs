use crate::http::{ParseError, Request, Response, StatusCode};
use crate::website_handler::WebsiteHandler;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

pub trait Handler {
    fn handle_request(&self, request: &Request) -> Response;

    fn handle_bad_request(&self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

// Implementation of the functionality of the struct
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    fn handle_stream(&self, stream: &mut TcpStream, handler: &WebsiteHandler) {
        // Do Sometgin with the stream
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {
                let response = match Request::try_from(&buffer[..]) {
                    Ok(request) => handler.handle_request(&request),
                    Err(e) => handler.handle_bad_request(&e),
                };

                if let Err(e) = response.send(stream) {
                    println!("Failed to parse stream {e}")
                }
            }
            Err(e) => {
                println!("Failed to read stream: {e}")
            }
        }
    }

    pub fn run(&self, handler: &WebsiteHandler) {
        println!("Server running on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap(); // Infinite loop. Can be labeled and break or continue with the label
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => self.handle_stream(&mut stream, handler),
                Err(error) => {
                    println!("Failed to establish a connection: {}", &error);
                    continue;
                } // Use _ to chatch any other option (i.e like default)
            }
        }
    }
}
