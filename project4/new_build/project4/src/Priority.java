/**
 * Non-preemptive priority scheduling algorithm.
 */
 
import java.util.*;


public class Priority implements Algorithm {
    private Queue<Task> queue;
    public Priority(List<Task> queue) {
        // implement priority scheduling algorithm
        // hint: use a queue
        this.queue = new LinkedList<Task>(queue);
        schedule();
    }
    public void schedule() {
        long startTime = System.currentTimeMillis();
        while (!queue.isEmpty()) {
            Task task = pickNextTask();
            CPU.run(task, task.getBurst());
            queue.remove(task);
        }
        long endTime = System.currentTimeMillis();
        System.out.println("Priority: " + (endTime - startTime));

    }
    public Task pickNextTask() {
        Task task = queue.peek();
        for (Task t : queue) {
            if (t.getPriority() < task.getPriority()) {
                task = t;
            }
        }
        return task;
    }
}