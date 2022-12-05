/**
 * FCFS scheduling algorithm.
 */
 
import java.util.*;

// import queue
import java.util.Queue;

public class FCFS implements Algorithm {
    public FCFS(List<Task> queue) {
        // create a queue of tasks
        Queue<Task> taskQueue = new LinkedList<Task>();
        for (Task task : queue) {
            taskQueue.add(task);
        }
        while (!taskQueue.isEmpty()) {
            Task task = taskQueue.remove();
            CPU.run(task, task.getBurst());
        }
    }
}