use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; 
    match stream.read(&mut buffer) {
        Ok(size) => {
            let sender = stream.peer_addr().unwrap();
            let received_data = String::from_utf8_lossy(&buffer[..size]);
            println!("received content from {}:{} : {}", sender.ip(), sender.port(), received_data);

            match String::from(received_data).as_str() {
                    "ping"  => {
                         match stream.write_all(b"alive") {
                            Ok(_) => {
                                println!("sended response for a ping request");
                            },
                            Err(e) => {
                                println!("error has occured while responding to a ping request {e}");
                            },
                         }
                },
                _ => {
                    
                },
            }
        }
        Err(e) => {
            eprintln!("Failed to read client's data : {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("cannot link port 8080");

    println!("server listening on 127.0.0.1:8080 ...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("client connection failed : {}", e);
            }
        }
    }
}
