/**
 * Non-preemptive priority scheduling algorithm using RR.
 *
 * This algorithm will run tasks according to round-robin scheduling.
 */
 
import java.util.*;

// Each process gets a small unit of CPU time (time quantum q),
// usually 10-100 milliseconds. After this time has elapsed, the
// process is preempted and added to the end of the ready queue.
// ! If there are n processes in the ready queue and the time
// quantum is q, then each process gets 1/n of the CPU time in
// chunks of at most q time units at once. No process waits more
// than (n-1)q time units.
// ! Timer interrupts every quantum to schedule next process
// ! Performance
// ! q large Þ FIFO
// ! q small Þ q must be large with respect to context switch,
// otherwise overhead is too high


public class RR implements Algorithm {
    public static final int QUANTUM = 10;
    private Queue<Task> queue;
    public RR(List<Task> taskQueue) {
        // implement Round Robin scheduling algorithm
        // each task gets QUANTUM time to run and then is preempted to the end of the queue so that the next task can run
        this.queue = new LinkedList<Task>(taskQueue);
        schedule();
    }
    
    public void schedule() {
        // start a timer which will measure and print the performance of the algorithm
        long startTime = System.currentTimeMillis();

        // while there are still tasks in the queue
        while (!queue.isEmpty()) {
            // get the next task
            Task task = pickNextTask();
            // ensure the remaining burst time is 0 or negative
            if (task.getBurst() <= 0) {
                // if so, remove the task from the queue
                queue.remove(task);
            } else {
                // otherwise, run the task for QUANTUM time
                CPU.run(task, QUANTUM);
                // decrement the task's burst time by QUANTUM
                task.setBurst(task.getBurst() - QUANTUM);
                queue.remove(task);
            }

            // if there is burst time remaining, add the task back to the end of the queue
            if (task.getBurst() > 0) {
                queue.add(task);
            }

            
        }
        // print the performance of the algorithm
        long endTime = System.currentTimeMillis();
        System.out.println("RR: " + (endTime - startTime));
        
    }
    
    public Task pickNextTask() {
        return queue.peek();
    }
}
