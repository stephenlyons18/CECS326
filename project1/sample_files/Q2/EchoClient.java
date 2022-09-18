/**
 * An echo client. The client enters data to the server, and the
 * server echoes the data back to the client.
 */
import java.net.*;
import java.io.*;

public class EchoClient
{
	//Your code is here
}


// Question 2:
// An echo server echoes back whatever it receives from a client. For example, if a client sends the
// server the string “Hello there! I am the client.”, the server will respond with “Hello there! I am
// the server.”
// Write an echo server (you can use the Java networking API). This server will wait for a client
// connection using the accept() method. When a client connection is received, the server will loop,
// performing the following steps:
// 1. Read data from the socket into a buffer.
// 2. Write the contents of the buffer back to the client.
// The server will break out of the loop only when it has determined that the client has closed the
// connection.
// Hints: the date server of Figure 1 uses the java.io.BufferedReader class. BufferedReader extends
// the java.io.Reader class, which is used for reading character streams. However, the echo server
// cannot guarantee that it will read characters from clients; it may receive binary data as well. The
// class java.io.InputStream deals with data at the byte level rather than the character level. Thus, your
// echo server must use an object that extends java.io.InputStream. The read() method in the
// java.io.InputStream class returns −1 when the client has closed its end of the socket connection.