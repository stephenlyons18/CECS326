/**
 * Non-preemptive priority scheduling algorithm.
 */
 
import java.util.*;

// Your code here
public class Priority {
    public static void run(List<Task> tasks) {
        // implement priority scheduling algorithm
        // hint: use a priority queue
        PriorityQueue<Task> taskQueue = new PriorityQueue<Task>(tasks.size(), new Comparator<Task>() {
            @Override
            public int compare(Task t1, Task t2) {
                return t1.getPriority() - t2.getPriority();
            }
        });
        for (Task task : tasks) {
            taskQueue.add(task);
        }
        while (!taskQueue.isEmpty()) {
            Task task = taskQueue.remove();
            CPU.run(task, task.getBurst());
        }
    }
}