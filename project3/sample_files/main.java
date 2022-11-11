import java.util.Random;

import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

class West_village extends Thread {
    private final Lock road;
    public West_village(Lock road) {
        this.road = road;
    }
    public void crossRoad() {
       // sleep for a random period between one and 5
        int sleepTime = new Random().nextInt(5) + 1;
        System.out.println("West Village took " + sleepTime + " seconds to cross the road");
        try {
                Thread.sleep(sleepTime * 1000);
        } catch (InterruptedException e) {
                e.printStackTrace();
        }
    }
    public West_village clone() {
        return new West_village(this.road);
    }
}

class East_village extends Thread {
    private final Lock road;
    public East_village(Lock road) {
       this.road = road;
    }
    public void crossRoad() {
        // sleep for a random period between one and 5
       int sleepTime = new Random().nextInt(5) + 1;
       System.out.println("East Village took " + sleepTime + " seconds to cross the road");
       try {
            Thread.sleep(sleepTime * 1000);
       } catch (InterruptedException e) {
          e.printStackTrace();
       }
    }
    public East_village clone() {
       return new East_village(this.road);
    }

 }

 class RoadController {
    private final Lock road;
    private final East_village east;
    private final West_village west;
    public RoadController() {
        this.road = new ReentrantLock();
        east = new East_village(road);
        west = new West_village(road);
    }
    public void crossRoad(String village) {
        synchronized (road) {
            switch (village) {
                case "east":
                    east.crossRoad();
                    break;
                case "west":
                    west.crossRoad();
                    break;
                default:
                    System.out.println("Invalid village");
            }
        }
    }
    public RoadController clone() {
        return new RoadController();
    }
}

public class main {
    public static void main(String[] args) {
        RoadController roadController = new RoadController();
        for (int i = 0; i < 10; i++) {
            roadController.crossRoad("east");
            roadController.crossRoad("west");
        }
    }

    // OUTPUT IS WRONG
    // USE RUST PROGRAM INSTEAD
}

