
Dining Philosopher Problem
(Github Repo)

EE 381 - Spring 2022
Stephen Lyons, 025785875
Noah Daniels, 026466906
Professor Hailu Xu



California State University, Long Beach
College of Engineering
1250 Bellflower Blvd, Long Beach, CA 90840
Due 10/15/2022



## Project Architecture

This project was programmed in the Rust programming language and the rust toolchain was used to compile and run the code. All classes and the main code are written in the main.rs file. The code can be run via the executable file or by installing the rust toolchain:

## To install Rust

1. Install Rustup: <https://rustup.rs/>
2. Install Rust stable: `rustup install stable`

## To run the project

1. `cd` into the project directory `project2rust`
2. `cargo run`
or
3. Run the executable in the target directory (project2rust\target\debug\project2rust.exe)


The Philosopher struct (class) has four fields: name, left_fork, right_fork, and eat_count. The name field is a string that holds the name of the philosopher. The left_fork and right_fork fields are mutexes that represent the forks. The Philosopher struct has two methods: eat and think. The eat method locks the left and right forks and then prints a message that the philosopher is eating. The think method unlocks the left and right forks and then prints a message that the philosopher is thinking. Both the thinking and eating functions take a random amount of time to simulate the philosopher thinking or eating, this is done by using the rand crate. Once the Philosopher is finished eating or thinking, it calls the other respective function

The Dining Server struct (class) has two fields: philosophers and forks. The philosophers field is a vector of Philosopher structs. The forks field is a vector of mutexes. The Dining Server implementation has two methods: take_forks and return_forks. The take_forks method takes a reference to a philosopher and locks the left and right forks. The return_forks method takes a reference to a philosopher and unlocks the left and right forks. 


The main function creates a new DiningServer, a vector of philosophers and forks and then spawns a thread for each philosopher. The main function then joins all the threads and prints the action of each philosopher and when they are done eating. 