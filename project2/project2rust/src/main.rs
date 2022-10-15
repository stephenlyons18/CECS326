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
        table.take_forks(self.index);
        // sleep for a random period between one and three seconds
        let mut sleep_time = rand::thread_rng().gen_range(1..4);
        thread::sleep(Duration::from_secs(sleep_time));
        // return forks after eating so other philosophers can eat
        table.return_forks(self.index);
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
        let mut sleep_time = rand::thread_rng().gen_range(1..4);
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
    fn take_forks(&self, philosopher_number: usize) {
        let left = philosopher_number;
        let right = (philosopher_number + 1) % 5;
        let _left = self.forks[left].lock().unwrap();

        // for some reason, these print statements break the code
        // println!("Fork #{} is locked by Philosopher #{}", left, philosopher_number);
        let _right = self.forks[right].lock().unwrap();

        // same as above, not sure why this breaks the code and causes a deadlock
        // println!("Fork #{} is locked by Philosopher #{}", right, philosopher_number);
        println!("Philosopher #{} took forks", philosopher_number);
    }
    // function to return forks which will unlock the mutexes for the left and right forks
    fn return_forks(&self, philosopher_number: usize) {
        let left = philosopher_number;
        let right = (philosopher_number + 1) % 5;
        drop(self.forks[left].lock().unwrap());
        drop(self.forks[right].lock().unwrap());
    }
}

fn main() {
    // create a new DiningServer with 5 forks which can be locked
    let table = Arc::new(DiningServer {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });
    // create a vector of philosophers with their names and forks
    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1, 0),
        Philosopher::new("Gilles Deleuze", 1, 2, 1),
        Philosopher::new("Karl Marx", 2, 3, 2),
        Philosopher::new("Emma Goldman", 3, 4, 3),
        Philosopher::new("Michel Foucault", 0, 4, 4),
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
