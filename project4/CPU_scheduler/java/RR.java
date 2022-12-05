/**
 * Non-preemptive priority scheduling algorithm using RR.
 *
 * This algorithm will run tasks according to round-robin scheduling.
 */
 
import java.util.*;

// Your code here

public class RR implements Algorithm {
    public RR(List<Task> queue) {
        // implement round-robin scheduling algorithm
        // hint: use a queue and a time quantum to divide the CPU time into slices
        Queue<Task> taskQueue = new LinkedList<Task>();
        for (Task task : queue) {
            taskQueue.add(task);
        }
        while (!taskQueue.isEmpty()) {
            Task task = taskQueue.remove();
            CPU.run(task, 10);
            if (task.getBurst() > 0) {
                taskQueue.add(task);
            }
        }

    }
}