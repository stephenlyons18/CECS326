/**
 * Quote server listening to port 6017.
 *
 */
 
import java.net.*;
import java.io.*;

public class QuoteServer
{
  public static void main(String[] args) throws IOException
  {
    String[] quotes = new String[5];
    quotes
        = new
        String
        [
        5
        ];
    quotes[0] = "You miss 100% of the shots you don't take. - Wayne Gretzky";
    quotes[1] = "You only live once, but if you do it right, once is enough. - Mae West";
    quotes[2] = "Be the change that you wish to see in the world. - Mahatma Gandhi";
    quotes[3] = "In three words I can sum up everything I've learned about life: it goes on. - Robert Frost";
    quotes[4] = "If you tell the truth, you don't have to remember anything. - Mark Twain";
    int port = 6017;
    try
    {
      ServerSocket serverSocket = new ServerSocket(port);
      while (true)
      {
        Socket client = serverSocket.accept();
        PrintWriter out = new PrintWriter(client.getOutputStream(), true);
        out.println(quotes[(int) (Math.random() * 5)]);
        client.close();
      }
    } catch (IOException e)
    {
      System.err.println("Could not listen on port: " + port);
      System.exit(-1);
    }

  }
  
}

