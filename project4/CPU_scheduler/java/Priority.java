/**
 * Non-preemptive priority scheduling algorithm.
 */
 
import java.util.*;

// Your code here
public class Priority implements Algorithm {
    public Priority(List<Task> queue) {
        // implement priority scheduling algorithm
        // hint: use a priority queue
        PriorityQueue<Task> taskQueue = new PriorityQueue<Task>(queue.size(), new Comparator<Task>() {
            @Override
            public int compare(Task t1, Task t2) {
                return t1.getPriority() - t2.getPriority();
            }
        });
        for (Task task : queue) {
            taskQueue.add(task);
        }
        while (!taskQueue.isEmpty()) {
            Task task = taskQueue.remove();
            CPU.run(task, task.getBurst());
        }
    }
}