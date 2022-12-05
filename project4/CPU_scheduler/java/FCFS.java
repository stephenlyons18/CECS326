/**
 * FCFS scheduling algorithm.
 */
 
import java.util.*;

// import queue
import java.util.Queue;

//Your code here

public class FCFS {
    public static void run(List<Task> tasks) {
        // implement FCFS scheduling algorithm using a queue
        
        Queue<Task> taskQueue = new LinkedList<Task>();
        for (Task task : tasks) {
            taskQueue.add(task);
        }
        while (!taskQueue.isEmpty()) {
            Task task = taskQueue.remove();
            CPU.run(task, task.getBurst());
        }
    }
}