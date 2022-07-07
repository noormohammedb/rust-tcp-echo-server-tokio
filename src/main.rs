use std::env::args;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{thread, time::Duration};

const ECHO_SERVER_ADDRESS: &str = "localhost:8000";
fn main() {
    // read arguments
    let delay = args()
        .nth(1)
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or_default();

    println!("delay: {}", delay);

    println!("TCP echo server starting tcp://{}", ECHO_SERVER_ADDRESS);

    // bind
    let listener = TcpListener::bind(ECHO_SERVER_ADDRESS).unwrap();

    // start
    println!("TCP echo server listening on {}", ECHO_SERVER_ADDRESS);

    for tcp_streams in listener.incoming() {
        let stream = tcp_streams.unwrap();

        handle_connection(stream, delay);
    }
}

fn handle_connection(mut stream: TcpStream, delay: u64) {
    // read the buffer
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("received message: {}", message);

    // delay
    thread::sleep(Duration::from_millis(delay));

    // write the message
    let _ = stream.write_all(message.as_bytes());
    println!("sent message: {}", message);
}
