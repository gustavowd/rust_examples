use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    loop {    
        let result = stream.read(&mut data);
        match result {
            Ok(size) => {
                // echo everything!
                if size > 0 {
                    stream.write(&data[0..size]).unwrap();
                }else {
                    println!("Connection closed: {}", stream.peer_addr().unwrap());
                    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
                    println!("Thread terminada!");
                    break;
                }
            },
            Err(_) => {
                //println!("size {}", result);
                println!("Resultado erro");
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
