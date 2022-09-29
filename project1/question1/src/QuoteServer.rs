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

// 3: The Required Deliverable Materials
// (1) A README file, which describes how we can compile and run your code.
// (2) Your source code, you may use a Makefile (for C) and be submitted in the required format.
// (3) Your short report, which discusses the design of your program.
// (4) A recorded video shows the output and runtime
// 3. Submission Requirements
// You need to strictly follow the instructions listed below:
// 1) This is a group project, please submit a .zip/.rar file that contains all files, only one submission
// from one group.
// 2) Make a video to record your code execution and outputs. The video should present your name
// or time as identification (You are suggested to upload the video to YouTube and put the link into
// your report).
// 3) The submission should include your source code and project report. Do not submit your binary
// code. Project report should contain your groupmates name and ID.
// 4) Your code must be able to compile; otherwise, you will receive a grade of zero.
// 5) Your code should not produce anything else other than the required information in the output
// file.
// 7) If you code is partially completed, please explain the details in the report what has been
// completed and the status of the missing parts, we will grade it based on the entire performance.
// 8) Provide sufficient comments in your code to help the TA understand your code. This is
// important for you to get at least partial credit in case your submitted code does not work properly.

use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// create a QuoteServer implementation that will be initialized and used in main.rs

// the quote server will have an array of quotes that will be randomly selected and sent to the client
// the quote server will also have a listener that will listen for incoming connections
pub struct QuoteServer {
    
    pub listener: TcpListener,
}

// implement the QuoteServer struct
impl QuoteServer {
    // create a list of quotes that will be randomly selected and sent to the client
    let quotes = ["The best way to predict the future is to invent it. - Alan Kay",
    "A computer lets you make more mistakes faster than any invention in human history, with the possible exceptions of handguns and tequila. - Mitch Ratcliffe",
    "The computer was born to solve problems that did not exist before. - Bill Gates",
    "A computer is like air conditioning: it becomes useless when you open Windows. - Linus Torvalds",
    "The most exciting phrase to hear in science, the one that heralds new discoveries, is not 'Eureka!' but 'That's funny...' - Isaac Asimov",
    "The best way to predict the future is to invent it. - Alan Kay",];

    // create a new QuoteServer
    pub fn new(listener: TcpListener) -> QuoteServer {
        QuoteServer { listener }
        // run the listen function
        listen();
    }

    // create a function that will randomly select a quote from the list of quotes
    pub fn select_quote(&mut self) -> String {
        let mut rng = rand::thread_rng();
        let quote = quotes.choose(&mut rng).unwrap();
        quote.to_string()
    }
    // create a function that will send a random quote to the client
    pub fn send_quote(&mut self) -> Result<String, Error> {
        let mut buffer = [0; 512];
        let bytes_read = self.stream.read(&mut buffer)?;
        let quote = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Quote: {}", quote);
    }

    // create a function that will listen for incoming connections
    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        let mut quote_server = QuoteServer::new(stream);
                        quote_server.send_quote();
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }


}