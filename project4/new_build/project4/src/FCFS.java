/**
 * FCFS scheduling algorithm.
 */
 
import java.util.*;

// import queue
import java.util.Queue;

public class FCFS implements Algorithm {
    private Queue<Task> queue;
    public FCFS(List<Task> queue) {
        // implement FCFS scheduling algorithm
        // hint: use a queue
        this.queue = new LinkedList<Task>(queue);

        schedule();
    }
    public void schedule() {
        long startTime = System.currentTimeMillis();
        while (!queue.isEmpty()) {
            Task task = queue.remove();
            CPU.run(task, task.getBurst());
        }

        long endTime = System.currentTimeMillis();
        System.out.println("FCFS: " + (endTime - startTime));
    }
    public Task pickNextTask() {
        return queue.peek();
    }

}