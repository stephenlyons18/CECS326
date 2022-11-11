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



struct RoadController {
    road: Arc<Mutex<()>>,
    east: EastVillage,
    west: WestVillage,
}
impl RoadController {
    fn new() -> RoadController {
        let road = Arc::new(Mutex::new(()));
        let east = EastVillage { road: road.clone() };
        let west = WestVillage { road: road.clone() };
        RoadController {
            road: road,
            east: east,
            west: west,
        }
    }
    pub fn cross_road(&self, village: &str) {
        match village {
            "east" => {
                let road = self.road.lock().unwrap();
                self.east.cross_road();
            }
            "west" => {
                let road = self.road.lock().unwrap();
                self.west.cross_road();
            }
            _ => println!("Invalid village"),
        }
    }
    pub fn clone(&self) -> RoadController {
        RoadController {
            road: self.road.clone(),
            east: self.east.clone(),
            west: self.west.clone(),
        }
    }
}
struct EastVillage {
    road: Arc<Mutex<()>>,
}
// implement EastVillage and WestVillage classes which both have access to the same road and if the road is occupied, the thread will wait until the road is free
impl EastVillage {
    fn new(road: Arc<Mutex<()>>) -> EastVillage {
        EastVillage { road: road }
    }
    pub fn cross_road(&self) {
        // sleep for a random period between one and 5
        let sleep_time = rand::thread_rng().gen_range(1..6);
        println!("East Village took {} seconds to cross the road", sleep_time);
        thread::sleep(Duration::from_secs(sleep_time));
    }
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
    fn new(road: Arc<Mutex<()>>) -> WestVillage {
        WestVillage { road: road }
    }
    pub fn cross_road(&self) {
        // sleep for a random period between one and 5
        let sleep_time = rand::thread_rng().gen_range(1..6);
        println!("West Village took {} seconds to cross the road", sleep_time);
        thread::sleep(Duration::from_secs(sleep_time));
    }
    pub fn clone(&self) -> WestVillage {
        WestVillage {
            road: self.road.clone(),
        }
    }
}

fn main() {
    let time = Instant::now();
    let road_controller = RoadController::new();
    let mut handles = vec![];
    let mut villageCount = 0;

    // create 10 threads
    for _ in 0..10 {
        let road_controller = road_controller.clone();
        let handle = thread::spawn(move || {
            // randomly choose a village
            let village = rand::thread_rng().gen_range(0..2);
            match village {
                0 => {
                    road_controller.cross_road("east");
                }
                1 => {
                    road_controller.cross_road("west");
                }
                _ => println!("Invalid village"),
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Time elapsed is: {:?}", time.elapsed());
}


