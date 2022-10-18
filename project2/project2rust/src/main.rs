use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
// import rand
use rand::Rng;
// define the philosopher struct
struct Philosopher {
    name: String,
    left: usize,
    right: usize,
    index: usize,
}
// define the function implementations for the philosopher struct
impl Philosopher {
    // constructor for the philosopher struct
    fn new(name: &str, left: usize, right: usize, index: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
            index: index,
        }
    }
    // function to eat which will call the take_forks and return_forks functions
    pub fn eat(&self, table: &DiningServer) {
        // sleep for a random period between one and three seconds
        let sleep_time = rand::thread_rng().gen_range(0..4);
        thread::sleep(Duration::from_secs(sleep_time));

        table.take_forks(self);
        // return forks after eating so other philosophers can eat
        table.return_forks(self);
        println!(
            "Philosopher #{} took {} seconds to eat",
            self.index, sleep_time
        );
        // switch the state of the philosopher to thinking
        self.think(&table);
    }
    // function to think which will only wait for a random period between one and three seconds and print
    pub fn think(&self, table: &DiningServer) {
        // sleep for a random period between one and three seconds
        let sleep_time = rand::thread_rng().gen_range(1..4);
        thread::sleep(Duration::from_secs(sleep_time));
        println!(
            "Philosopher #{} took {} seconds to think",
            self.index, sleep_time
        );
        // switch the state of the philosopher to eating
        self.eat(&table);
    }
}
// define the DiningServer struct
struct DiningServer {
    forks: Vec<Mutex<()>>,
}
// define the function implementations for the DiningServer struct
impl DiningServer {
    // constructor for the DiningServer struct
    fn new() -> DiningServer {
        let forks = vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ];
        DiningServer { forks }
    }
    // function to take forks which will lock the mutexes for the left and right forks
    fn take_forks(&self, philosopher: &Philosopher) {
        // bitwise AND operator to check if the index of the philosopher is even or odd
        if philosopher.index & 1 == 0 {
            // if the index is even, lock the left fork first
            let _left = self.forks[philosopher.left].lock().unwrap();
            // then lock the right fork
            thread::sleep(Duration::from_secs(1));
            let _right = self.forks[philosopher.right].lock().unwrap();
            println!(
                "Fork #{} is locked by Philosopher #{}",
                philosopher.left, philosopher.index
            );
            println!(
                "Fork #{} is locked by Philosopher #{}",
                philosopher.right, philosopher.index
            );
            println!("Philosopher #{} took forks", philosopher.index);
        } else {
            // if the index is odd, lock the right fork first
            let _right = self.forks[philosopher.right].lock().unwrap();
            thread::sleep(Duration::from_secs(1));
            let _left = self.forks[philosopher.left].lock().unwrap();
            println!(
                "Fork #{} is locked by Philosopher #{}",
                philosopher.right, philosopher.index
            );
            println!(
                "Fork #{} is locked by Philosopher #{}",
                philosopher.left, philosopher.index
            );
            println!("Philosopher #{} took forks", philosopher.index);
        }
    }
    // function to return forks which will unlock the mutexes for the left and right forks
    fn return_forks(&self, philosopher: &Philosopher) {
        drop(self.forks[philosopher.left].lock().unwrap());
        drop(self.forks[philosopher.right].lock().unwrap());
    }
}

fn main() {
    // create a new DiningServer with 5 forks which can be locked
    let table = Arc::new(DiningServer::new());
    // create a vector of philosophers with their names and forks
    let philosophers = vec![
        Philosopher::new("Philosopher 0", 0, 1, 0),
        Philosopher::new("Philosopher 1", 1, 2, 1),
        Philosopher::new("Philosopher 2", 2, 3, 2),
        Philosopher::new("Philosopher 3", 3, 4, 3),
        Philosopher::new("Philosopher 4", 0, 4, 4),
    ];
    // create a vector of threads to hold the philosophers
    let handles: Vec<_> = philosophers
        // iterate through the philosophers
        .into_iter()
        // map the philosophers to a thread
        .map(|p| {
            // clone the table so each philosopher can access it
            let table = table.clone();
            // create a new thread for each philosopher
            thread::spawn(move || {
                // call the eat function for each philosopher to start the dining process and loop forever between eating and thinking
                p.eat(&table);
            })
        })
        // collect the threads into a vector
        .collect();
    // for each thread in the vector of threads, join the thread
    for h in handles {
        h.join().unwrap();
    }
}
