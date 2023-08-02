use std::{net::TcpStream, io::{Write, Error}, io::Read, thread, time};

fn ping(addrs: &str, port: u16, count: i32) -> Result<(), std::io::Error> {
    let send_request = |addrs: &str, port: u16| -> Result<String, std::io::Error> {
        match TcpStream::connect((addrs, port)) {
           Ok(mut s) => {
                s.write_all(b"ping")?;
                let mut buffer = [0u8; 1024];
                match s.read(&mut buffer) {
                    Ok(r) => {
                        s.shutdown(std::net::Shutdown::Both)?;
                        return Ok(String::from(String::from_utf8_lossy(&buffer[..r]))) 
                    }, 
                    Err(_) => {
                        s.shutdown(std::net::Shutdown::Both)?;
                        return Err(Error::new(std::io::ErrorKind::Other, format!("ping request to {addrs}:{port} has failed.")))
                    },
                };  
           },
           Err(_) => {
                return Err(Error::new(std::io::ErrorKind::ConnectionRefused, format!("connection to {addrs}:{port} has failed."))) 
           },
        };
    }; 
    match count {
        ..=-1 => {
            loop {
                match send_request(addrs, port) {
                    Ok(r) => {
                        match r.as_str() {
                            "" => {
                                println!("ping request was a success, but remote did not returned any message.");
                            },
                            _ => {
                                println!("ping success (reponse from remote: {r})"); 
                            },
                        } 
                    },
                    Err(e) => {
                        println!("{e}");     
                    },
                }             
                thread::sleep(time::Duration::from_secs(1));
            } 
        },
        0 => {
           println!("cannot ping 0 time"); 
        },
        _ => {
            let mut success: i32 = 0;
            let mut failed: i32 = 0;
             for i in 0..count {
                 match send_request(addrs, port) {
                   Ok(r) => {
                        success += 1;
                        match r.as_str() {
                            "" => {println!("({i}) ping request was a success, but remote did not returned any message.");},
                            _ => {println!("ping success (response from remote: {r})")},
                        }
                   },
                   Err(e) => {
                       failed += 1;
                       println!("{e}")
                   },
                 }
                 thread::sleep(time::Duration::from_secs(1));
             }
             println!("performed {count} ping request to {addrs}:{port} -> success: {success} ; failed: {failed} ({}% success / {}% fail)", success/count*100, failed/count*100);
        }
    }
    Ok(())
}

fn main() {
    let addrs = "127.0.0.1";
    let port = 8080;
    let count = 3;

    ping(addrs, port, count).expect("Ping request has failed");
}
