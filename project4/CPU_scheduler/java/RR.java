/**
 * Non-preemptive priority scheduling algorithm using RR.
 *
 * This algorithm will run tasks according to round-robin scheduling.
 */
 
import java.util.*;

// Your code here

public class RR  {
    public static void run(List<Task> tasks, int quantum) {
        // implement round robin scheduling algorithm
        // hint: use a queue
        Queue<Task> taskQueue = new LinkedList<Task>();
        for (Task task : tasks) {
            taskQueue.add(task);
        }
        while (!taskQueue.isEmpty()) {
            Task task = taskQueue.remove();
            CPU.run(task, quantum);
            if (task.getBurst() > 0) {
                taskQueue.add(task);
            }
        }

    }
}