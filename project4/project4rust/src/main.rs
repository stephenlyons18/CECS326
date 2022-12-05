// Group Project 4: CPU scheduler
// CECS 326 – Operating Systems
// 1. Summary
// This project follows the topic of CPU scheduling, where it requires to design and implement several
// classic CPU scheduling algorithms.
// You should submit the required deliverable materials on BeachBoard by 11:55pm, December 4th
// (Sunday), 2022.
// 2. Description
// This project involves implementing several different process scheduling algorithms. The scheduler
// will be assigned a predefined set of tasks and will schedule the tasks based on the selected
// scheduling algorithm. Each task is assigned a priority and CPU burst. The following scheduling
// algorithms will be implemented:
// • First-come, first-served (FCFS), which schedules tasks in the order in which they request
// the CPU.
// • Priority scheduling, which schedules tasks based on priority.
// • Round-robin (RR) scheduling, where each task is run for a time quantum (or for the
// remainder of its CPU burst).
// Priorities range from 1 to 10, where a higher numeric value indicates a higher relative priority. For
// round-robin scheduling, the length of a time quantum is 10 milliseconds.
// You need to complete the three scheduling algorithms in the attached sample code (schedule_fcfs.c,
// schedule_priority.c, schedule_rr.c) or (FCFS.java, Priority.java, RR.java).
// 3. Implementation
// The implementation of this project should be completed in C or java, and supported program files
// are provided in the sample code. These supporting files read in the schedule of tasks, insert the
// tasks into a list, and invoke the scheduler.
// The schedule of tasks has the form [task name] [priority] [CPU burst], with the following
// example format:
// T1, 4, 20
// T2, 2, 25
// T3, 3, 25
// T4, 3, 15
// T5, 10, 10
// Thus, task T1 has priority 4 and a CPU burst of 20 milliseconds, and so forth. It is assumed that all
// tasks arrive at the same time, so your scheduler algorithms do not have to support higher-priority
// processes preempting processes with lower priorities. In addition, tasks do not have to be placed
// into a queue or list in any particular order.
// There are a few different strategies for organizing the list of tasks. One approach is to place all
// tasks in a single unordered list, where the strategy for task selection depends on the scheduling
// algorithm. Alternatively, a list could be ordered according to scheduling criteria (that is, by priority).
// One other strategy involves having a separate queue for each unique priority. It is also worth
// highlighting that we are using the terms list and queue somewhat interchangeably. However, a
// queue has very specific FIFO functionality, whereas a list does not have such strict insertion and
// deletion requirements. You are likely to find the functionality of a general list to be more suitable
// when completing this project.
// 4. C Implementation Details (You can also use the java repo)
// The file driver.c reads in the schedule of tasks, inserts each task into a linked list, and invokes the
// process scheduler by calling the schedule() function. The schedule() function executes each task
// according to the specified scheduling algorithm. Tasks selected for execution on the CPU are
// determined by the pickNextTask() function and are executed by invoking the run() function
// defined in the CPU.c file. A Makefile is used to determine the specific scheduling algorithm that
// will be invoked by driver. For example, to build the FCFS scheduler, we would enter
// make fcfs
// and would execute the scheduler (using the schedule of tasks schedule.txt) as follows:
// ./fcfs schedule.txt
// Refer to the Makefile in the source code download for further details. Before proceeding, be sure
// to familiarize yourself with the source code provided as well as the Makefile.
// 3: The Required Deliverable Materials
// (1) Your source code, must be submitted in the required format.
// (2) Your report, which discusses the design of your program. Report should display the outputs of
// different scheduling algorithms from your code.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub struct Task {
    name: String,
    tid: AtomicUsize,
    priority: i32,
    burst: i32,
}

impl Task {
    pub fn new(name: String, priority: i32, burst: i32) -> Task {
        Task {
            name: name,
            tid: AtomicUsize::new(0),
            priority: priority,
            burst: burst,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_tid(&self) -> usize {
        self.tid.load(Ordering::Relaxed)
    }

    pub fn get_priority(&self) -> i32 {
        self.priority
    }

    pub fn get_burst(&self) -> i32 {
        self.burst
    }

    pub fn set_priority(&mut self, priority: i32) -> i32 {
        self.priority = priority;
        priority
    }

    pub fn set_burst(&mut self, burst: i32) -> i32 {
        self.burst = burst;
        burst
    }

    pub fn equals(&self, other: &Task) -> bool {
        self.tid.load(Ordering::Relaxed) == other.tid.load(Ordering::Relaxed)
    }

    pub fn to_string(&self) -> String {
        format!(
            "Name: {}\nTid: {}\nPriority: {}\nBurst: {}\n",
            self.name,
            self.tid.load(Ordering::Relaxed),
            self.priority,
            self.burst
        )
    }
}

pub trait Algorithm {
    fn schedule(&mut self);
    fn pick_next_task(&mut self) -> Task;
}
// impl Algorithm for Round Robin scheduling algorithm 
pub fn RR(Vec<Task>)  {
    let mut tasks = Vec::new();
    let mut time = 0;
    let mut time_quantum = 10;
    let mut current_task = 0;
    let mut task_count = 0;
    
    // loop through the tasks and execute them in small bursts of time, switching between them as the time quantum expires
    for task in tasks {
        while task.get_burst() > 0 {
            if task.get_burst() > time_quantum {
                time += time_quantum;
                task.set_burst(task.get_burst() - time_quantum);
            } else {
                time += task.get_burst();
                task.set_burst(0);
            }
            println!("Task {} executed for {} ms", task.get_name(), time);
            current_task += 1;
            if current_task == task_count {
                current_task = 0;
            }
        }
    }
}

pub fn FCFS(Vec<Task>) {
    let mut tasks = Vec::new();
    let mut time = 0;
    let mut current_task = 0;
    let mut task_count = 0;
    
    // loop through the tasks and execute them in order of arrival
    for task in tasks {
        while task.get_burst() > 0 {
            time += task.get_burst();
            task.set_burst(0);
            println!("Task {} executed for {} ms", task.get_name(), time);
            current_task += 1;
            if current_task == task_count {
                current_task = 0;
            }
        }
    }
}
// implement priority scheduling algorithm which takes a list of tasks as input and executes them in order of priority
pub fn PS(Vec<Task>) {
    let mut tasks = Vec::new();
    let mut time = 0;
    let mut current_task = 0;
    let mut task_count = 0;
    
    // sort the tasks by priority
    tasks.sort_by(|a, b| a.get_priority().cmp(&b.get_priority()));

    // loop through the tasks and execute them in order of priority
    for task in tasks {
        while task.get_burst() > 0 {
            time += task.get_burst();
            task.set_burst(0);
            println!("Task {} executed for {} ms", task.get_name(), time);
            current_task += 1;
            if current_task == task_count {
                current_task = 0;
            }
        }
    }
    
}

fn main() {
    // use the rr-schedule file in the current directory to test the round-robin scheduling algorithm and measure its time performance using the time command
    let mut taskQueue = Vec::<Task>::new();
    let mut file = File::open("rr-schedule").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();
    for line in lines {
        let mut iter = line.split_whitespace();
        let name = iter.next().unwrap();
        let priority = iter.next().unwrap().parse::<i32>().unwrap();
        let burst = iter.next().unwrap().parse::<i32>().unwrap();
        taskQueue.push(Task::new(name.to_string(), priority, burst));
    }
    // measure the time performance of the round-robin scheduling algorithm
    let start = Instant::now();
    RR(taskQueue);
    let duration = start.elapsed();
    println!("Time elapsed in RR() is: {:?}", duration);

    // use the schedule.txt file in the current directory to test the first-come, first-served scheduling algorithm and measure its time performance using the time command
    let mut taskQueue = Vec::<Task>::new();
    let mut file = File::open("schedule.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();
    for line in lines {
        let mut iter = line.split_whitespace();
        let name = iter.next().unwrap();
        let priority = iter.next().unwrap().parse::<i32>().unwrap();
        let burst = iter.next().unwrap().parse::<i32>().unwrap();
        taskQueue.push(Task::new(name.to_string(), priority, burst));
    }
    // measure the time performance of the first-come, first-served scheduling algorithm
    let start = Instant::now();
    FCFS(taskQueue);
    let duration = start.elapsed();
    println!("Time elapsed in FCFS() is: {:?}", duration);

    // use the 1-sched.txt file in the current directory to test the priority scheduling algorithm and measure its time performance using the time command
    let mut taskQueue = Vec::<Task>::new();
    let mut file = File::open("1-sched.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();
    for line in lines {
        let mut iter = line.split_whitespace();
        let name = iter.next().unwrap();
        let priority = iter.next().unwrap().parse::<i32>().unwrap();
        let burst = iter.next().unwrap().parse::<i32>().unwrap();
        taskQueue.push(Task::new(name.to_string(), priority, burst));
    }
    // measure the time performance of the priority scheduling algorithm
    let start = Instant::now();
    Priority(taskQueue);
    let duration = start.elapsed();
    println!("Time elapsed in Priority() is: {:?}", duration);
}
