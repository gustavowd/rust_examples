use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{thread, time};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ip_addr = "localhost:3333".to_string();
    if args.len() == 2 {
        let ip_addr_str = &args[1];
        ip_addr.clear();
        ip_addr.push_str(ip_addr_str);
        ip_addr.push_str(":3333");
    }
    let ip_addr_str = ip_addr.clone();

    match TcpStream::connect(ip_addr) {
        Ok(mut stream) => {
            println!("Successfully connected to server in IP {}", ip_addr_str);

            let msg = b"Hello!";
            let ten_millis = time::Duration::from_millis(2000);
            //let now = time::Instant::now();

            loop {
                stream.write(msg).unwrap();
                println!("Sent Hello, awaiting reply...");

                let mut data = [0 as u8; 6]; // using 6 byte buffer
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        if &data == msg {
                            println!("Reply is ok!");
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("Unexpected reply: {}", text);
                        }
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
                thread::sleep(ten_millis);
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
