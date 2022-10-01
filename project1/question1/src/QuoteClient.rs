// instructions:
// Group Project 1: Warm up of Interprocess Communication
// CECS 326 – Operating Systems
// You should submit the required deliverable materials on BeachBoard by 11:55pm, September
// 29th (Thursday), 2022. We provide sample code files that you can complete the required functions.
// 1. Description
// Question 1:
// Sockets lecture describes certain port numbers as being well known—that is, they provide standard
// services. Port 17 is known as the quote-of-the-day service. When a client connects to port 17 on a
// server, the server responds with a quote for that day.
// Modify the date server shown in Figure 1 so that it delivers a quote of the day rather than the current
// date. The quotes should be printable ASCII characters and should contain fewer than 512 characters,
// although multiple lines are allowed. Since these well-known ports are reserved and therefore
// unavailable, have your server listen to port 6017. The date client can be used to read the quotes
// returned by your server.
// Figure 1. Data Server
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// create a QuoteClient implementation that will be initialized and used in main.rs
impl QuoteClient {
    // create a new QuoteClient
    pub fn new(stream: TcpStream) -> QuoteClient {
        QuoteClient { stream }
    }

    // read the quote from the QuoteServer
    pub fn read_quote(&mut self) {
        let mut buffer = [0; 512];
        match self.stream.read(&mut buffer) {
            Ok(_) => {
                let quote = String::from_utf8_lossy(&buffer[..]);
                println!("Quote: {}", quote);
            }
            Err(e) => println!("Failed to read from stream: {}", e),
        }
    }
}
