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


struct task {
    char name[10];
    int priority;
    int burst;
    struct task *next;
}

// implement FCFS scheduling algorithm using a queue
struct FCFS{
    // using a queue
    struct task *head;
    struct task *tail;
}
main(){
    // do nothing
    
}