// 1. Problem Description
// Consider that has this kind of scenario: two villages (Eastvillage and Westvillage) only have a
// single-lane road for the connection. People from these two villages only can use this road for
// exchange or share their produce. The road can be deadlocked if a people from either East or West
// on the road simultaneously. To solve this problem to avoid deadlock, please design an algorithm
// that uses semaphores and/or mutex locks. There have no concerns for the starvation cases.
// Implement your solution using synchronization tools. In particular, represent the people at
// Eastvillage and Westvillage as separate threads (east_village.java and west_village.java). Once a
// people is on the road, the associated thread will sleep for a random period of time, representing
// traveling across the road. You should design a new action for each people when they get into the
// road, such as eat a donut, to wait for some time. Design your program so that you can create several
// threads representing the two villages’ people without deadlock in the road.
// You can flexibly design the algorithm, the test should have no deadlock in the multiple execution
// cases.
// 2. The Required Deliverable Materials
// (1) A README file, which describes how we can compile and run your code.
// (2) Your source code, should submit in the required format.
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
use std::time::Duration;
use std::time::Instant;

use std::sync::{Arc, Mutex};
use std::thread;
// import rand
use rand::Rng;

// define the struct of the RoadController which contains ONE mutex lock and the two villages
struct RoadController {
    road: Arc<Mutex<()>>,
    east: EastVillage,
    west: WestVillage,
}
impl RoadController {
    // RoadController constructor
    fn new() -> RoadController {
        // create a new mutex lock to be shared by the two villages
        let road = Arc::new(Mutex::new(()));
        // create the two villages and pass the mutex lock to them
        let east = EastVillage { road: road.clone() };
        let west = WestVillage { road: road.clone() };
        // return the RoadController as the new constructed object
        RoadController {
            road: road,
            east: east,
            west: west,
        }
    }
    //
    pub fn cross_road(&self, village: &str) {
        // based on the village name, lock the mutex lock
        match village {
            "east" => {
                // use a guard to make sure the mutex lock is unlocked before it begins to lock again
                let _guard = self.road.lock().unwrap();
                // lock the mutex lock execute the crossing road action for the east village
                self.east.cross_road();
            }
            "west" => {
                // use a guard to make sure the mutex lock is unlocked before it begins to lock again
                let _guard = self.road.lock().unwrap();
                // lock the mutex lock execute the crossing road action for the west village
                self.west.cross_road();
            }
            // otherwise, print the error message since the village name is not valid
            _ => println!("Invalid village"),
        }
    }
    // copy constructor for the RoadController
    pub fn clone(&self) -> RoadController {
        RoadController {
            road: self.road.clone(),
            east: self.east.clone(),
            west: self.west.clone(),
        }
    }
}
// define the struct of the EastVillage which contains the mutex lock created by the RoadController
struct EastVillage {
    road: Arc<Mutex<()>>,
}


// implement EastVillage and WestVillage classes which both have access to the same road and if the road is occupied, the thread will wait until the road is free
impl EastVillage {
    // cross_road function which will be called by the RoadController to cross the road and sleep for a random time
    pub fn cross_road(&self) {
        // generate a random number between 1 and 5 to represent the time it takes to cross the road
        let sleep_time = rand::thread_rng().gen_range(1..6);
        println!("East Village took {} seconds to cross the road", sleep_time);
        // make the thread sleep for the random time
        thread::sleep(Duration::from_secs(sleep_time));
    }
    // copy constructor for the EastVillage
    pub fn clone(&self) -> EastVillage {
        EastVillage {
            road: self.road.clone(),
        }
    }
}
struct WestVillage {
    road: Arc<Mutex<()>>,
}
impl WestVillage {
    // cross_road function which will be called by the RoadController to cross the road and sleep for a random time
    pub fn cross_road(&self) {
        // sleep for a random period between one and five seconds to represent the time it takes to cross the road
        let sleep_time = rand::thread_rng().gen_range(1..6);
        println!("West Village took {} seconds to cross the road", sleep_time);
        // make the thread sleep for the random time
        thread::sleep(Duration::from_secs(sleep_time));
    }
    // copy constructor for the WestVillage
    pub fn clone(&self) -> WestVillage {
        WestVillage {
            road: self.road.clone(),
        }
    }
}

fn main() {
    // initialize a timer to measure the time it takes to execute the program
    let time = Instant::now();

    // create a new RoadController
    let road_controller = RoadController::new();
    // create a vector to store all the threads that will be created
    let mut handles = vec![];

    // create 10 threads
    for _ in 0..10 {
        // clone the RoadController to be passed to the thread
        let road_controller = road_controller.clone();
        // create a new thread
        let handle = thread::spawn(move || {
            // randomly choose a village
            let village = rand::thread_rng().gen_range(0..2);

            match village {
                0 => {
                    // if the village is 0, then the east village will cross the road
                    road_controller.cross_road("east");
                }
                1 => {
                    // otherwise, the west village will cross the road
                    road_controller.cross_road("west");
                }
                _ => println!("Invalid village"),
            }
        });
        // push the thread to the vector of threads
        handles.push(handle);
    }
    // join all the threads to the main thread such that the main thread will wait until all the threads are finished
    for handle in handles {
        handle.join().unwrap();
    }
    // print the time it takes to execute the program
    println!("Time elapsed is: {:?}", time.elapsed());
}
