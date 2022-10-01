// TCP LISTENER DOCUMENTATION
// Struct std::net::TcpListenerCopy item path
// 1.0.0 · source · [−]
// pub struct TcpListener(_);
// A TCP socket server, listening for connections.

// After creating a TcpListener by binding it to a socket address, it listens for incoming TCP connections. These can be accepted by calling accept or by iterating over the Incoming iterator returned by incoming.

// The socket will be closed when the value is dropped.

// The Transmission Control Protocol is specified in IETF RFC 793.

// Examples
// use std::net::{TcpListener, TcpStream};

// fn handle_client(stream: TcpStream) {
//     // ...
// }

// fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:80")?;

//     // accept connections and process them serially
//     for stream in listener.incoming() {
//         handle_client(stream?);
//     }
//     Ok(())
// }
// Implementations
// source
// impl TcpListener
// source
// pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<TcpListener>
// Creates a new TcpListener which will be bound to the specified address.

// The returned listener is ready for accepting connections.

// Binding with a port number of 0 will request that the OS assigns a port to this listener. The port allocated can be queried via the TcpListener::local_addr method.

// The address type can be any implementor of ToSocketAddrs trait. See its documentation for concrete examples.

// If addr yields multiple addresses, bind will be attempted with each of the addresses until one succeeds and returns the listener. If none of the addresses succeed in creating a listener, the error returned from the last attempt (the last address) is returned.

// Examples
// Creates a TCP listener bound to 127.0.0.1:80:

// use std::net::TcpListener;

// let listener = TcpListener::bind("127.0.0.1:80").unwrap();
// Creates a TCP listener bound to 127.0.0.1:80. If that fails, create a TCP listener bound to 127.0.0.1:443:

// use std::net::{SocketAddr, TcpListener};

// let addrs = [
//     SocketAddr::from(([127, 0, 0, 1], 80)),
//     SocketAddr::from(([127, 0, 0, 1], 443)),
// ];
// let listener = TcpListener::bind(&addrs[..]).unwrap();
// source
// pub fn local_addr(&self) -> Result<SocketAddr>
// Returns the local socket address of this listener.

// Examples
// use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};

// let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
// assert_eq!(listener.local_addr().unwrap(),
//            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)));
// source
// pub fn try_clone(&self) -> Result<TcpListener>
// Creates a new independently owned handle to the underlying socket.

// The returned TcpListener is a reference to the same socket that this object references. Both handles can be used to accept incoming connections and options set on one listener will affect the other.

// Examples
// use std::net::TcpListener;

// let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
// let listener_clone = listener.try_clone().unwrap();
// source
// pub fn accept(&self) -> Result<(TcpStream, SocketAddr)>
// Accept a new incoming connection from this listener.

// This function will block the calling thread until a new TCP connection is established. When established, the corresponding TcpStream and the remote peer’s address will be returned.

// Examples
// use std::net::TcpListener;

// let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
// match listener.accept() {
//     Ok((_socket, addr)) => println!("new client: {addr:?}"),
//     Err(e) => println!("couldn't get client: {e:?}"),
// }
// source
// pub fn incoming(&self) -> Incoming<'_>ⓘ
// Returns an iterator over the connections being received on this listener.

// The returned iterator will never return None and will also not yield the peer’s SocketAddr structure. Iterating over it is equivalent to calling TcpListener::accept in a loop.

// Examples
// use std::net::{TcpListener, TcpStream};

// fn handle_connection(stream: TcpStream) {
//    //...
// }

// fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:80").unwrap();

//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 handle_connection(stream);
//             }
//             Err(e) => { /* connection failed */ }
//         }
//     }
//     Ok(())
// }
// source
// pub fn into_incoming(self) -> IntoIncomingⓘ
// 🔬
//  This is a nightly-only experimental API. (
// tcplistener_into_incoming

// #88339
// )
// Turn this into an iterator over the connections being received on this listener.

// The returned iterator will never return None and will also not yield the peer’s SocketAddr structure. Iterating over it is equivalent to calling TcpListener::accept in a loop.

// Examples
// #![feature(tcplistener_into_incoming)]
// use std::net::{TcpListener, TcpStream};

// fn listen_on(port: u16) -> impl Iterator<Item = TcpStream> {
//     let listener = TcpListener::bind("127.0.0.1:80").unwrap();
//     listener.into_incoming()
//         .filter_map(Result::ok) /* Ignore failed connections */
// }

// fn main() -> std::io::Result<()> {
//     for stream in listen_on(80) {
//         /* handle the connection here */
//     }
//     Ok(())
// }
// 1.9.0 · source
// pub fn set_ttl(&self, ttl: u32) -> Result<()>
// Sets the value for the IP_TTL option on this socket.

// This value sets the time-to-live field that is used in every packet sent from this socket.

// Examples
// use std::net::TcpListener;

// let listener = TcpListener::bind("127.0.0.1:80").unwrap();
// listener.set_ttl(100).expect("could not set TTL");
// 1.9.0 · source
// pub fn ttl(&self) -> Result<u32>
// Gets the value of the IP_TTL option for this socket.

// For more information about this option, see TcpListener::set_ttl.

// Examples
// use std::net::TcpListener;

// let listener = TcpListener::bind("127.0.0.1:80").unwrap();
// listener.set_ttl(100).expect("could not set TTL");
// assert_eq!(listener.ttl().unwrap_or(0), 100);
// 1.9.0 · source
// pub fn set_only_v6(&self, only_v6: bool) -> Result<()>
// 👎
//  Deprecated since 1.16.0:
// this option can only be set before the socket is bound

// 1.9.0 · source
// pub fn only_v6(&self) -> Result<bool>
// 👎
//  Deprecated since 1.16.0:
// this option can only be set before the socket is bound

// 1.9.0 · source
// pub fn take_error(&self) -> Result<Option<Error>>
// Gets the value of the SO_ERROR option on this socket.

// This will retrieve the stored error in the underlying socket, clearing the field in the process. This can be useful for checking errors between calls.

// Examples
// use std::net::TcpListener;

// let listener = TcpListener::bind("127.0.0.1:80").unwrap();
// listener.take_error().expect("No error was expected");
// 1.9.0 · source
// pub fn set_nonblocking(&self, nonblocking: bool) -> Result<()>
// Moves this TCP stream into or out of nonblocking mode.

// This will result in the accept operation becoming nonblocking, i.e., immediately returning from their calls. If the IO operation is successful, Ok is returned and no further action is required. If the IO operation could not be completed and needs to be retried, an error with kind io::ErrorKind::WouldBlock is returned.

// On Unix platforms, calling this method corresponds to calling fcntl FIONBIO. On Windows calling this method corresponds to calling ioctlsocket FIONBIO.

// Examples
// Bind a TCP listener to an address, listen for connections, and read bytes in nonblocking mode:

// use std::io;
// use std::net::TcpListener;

// let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
// listener.set_nonblocking(true).expect("Cannot set non-blocking");

// for stream in listener.incoming() {
//     match stream {
//         Ok(s) => {
//             // do something with the TcpStream
//             handle_connection(s);
//         }
//         Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
//             // wait until network socket is ready, typically implemented
//             // via platform-specific APIs such as epoll or IOCP
//             wait_for_fd();
//             continue;
//         }
//         Err(e) => panic!("encountered IO error: {e}"),
//     }
// }
// Trait Implementations
// 1.63.0 · source
// impl AsFd for TcpListener
// source
// fn as_fd(&self) -> BorrowedFd<'_>
// Available on
// Unix
//  only.
// Borrows the file descriptor. Read more

// source
// impl AsRawFd for TcpListener
// source
// fn as_raw_fd(&self) -> RawFd
// Available on
// Unix
//  only.
// Extracts the raw file descriptor. Read more

// source
// impl AsRawSocket for TcpListener
// Available on
// Windows
//  only.
// source
// fn as_raw_socket(&self) -> RawSocket
// Extracts the raw socket. Read more

// 1.63.0 · source
// impl AsSocket for TcpListener
// Available on
// Windows
//  only.
// source
// fn as_socket(&self) -> BorrowedSocket<'_>
// Borrows the socket.

// source
// impl Debug for TcpListener
// source
// fn fmt(&self, f: &mut Formatter<'_>) -> Result
// Formats the value using the given formatter. Read more

// 1.63.0 · source
// impl From<OwnedFd> for TcpListener
// source
// fn from(owned_fd: OwnedFd) -> Self
// Converts to this type from the input type.

// 1.63.0 · source
// impl From<OwnedSocket> for TcpListener
// Available on
// Windows
//  only.
// source
// fn from(owned: OwnedSocket) -> Self
// Converts to this type from the input type.

// 1.63.0 · source
// impl From<TcpListener> for OwnedFd
// source
// fn from(tcp_listener: TcpListener) -> OwnedFd
// Converts to this type from the input type.

// 1.63.0 · source
// impl From<TcpListener> for OwnedSocket
// Available on
// Windows
//  only.
// source
// fn from(tcp_listener: TcpListener) -> OwnedSocket
// Converts to this type from the input type.

// 1.1.0 · source
// impl FromRawFd for TcpListener
// source
// unsafe fn from_raw_fd(fd: RawFd) -> TcpListener
// Available on
// Unix
//  only.
// Constructs a new instance of Self from the given raw file descriptor. Read more

// 1.1.0 · source
// impl FromRawSocket for TcpListener
// Available on
// Windows
//  only.
// source
// unsafe fn from_raw_socket(sock: RawSocket) -> TcpListener
// Constructs a new I/O object from the specified raw socket. Read more

// 1.4.0 · source
// impl IntoRawFd for TcpListener
// source
// fn into_raw_fd(self) -> RawFd
// Available on
// Unix
//  only.
// Consumes this object, returning the raw underlying file descriptor. Read more

// 1.4.0 · source
// impl IntoRawSocket for TcpListener
// Available on
// Windows
//  only.
// source
// fn into_raw_socket(self) -> RawSocket
// Consumes this object, returning the raw underlying socket. Read more

// source
// impl TcpListenerExt for TcpListener
// Available on
// WASI
//  only.
// source
// fn sock_accept(&self, flags: u16) -> Result<u32>
// 🔬
//  This is a nightly-only experimental API. (
// wasi_ext

// #71213
// )
// Accept a socket. Read more

// Auto Trait Implementations
// impl RefUnwindSafe for TcpListener
// impl Send for TcpListener
// impl Sync for TcpListener
// impl Unpin for TcpListener
// impl UnwindSafe for TcpListener
// Blanket Implementations
// source
// impl<T> Any for T
// where
//     T: 'static + ?Sized,
// source
// impl<T> Borrow<T> for T
// where
//     T: ?Sized,
// source
// impl<T> BorrowMut<T> for T
// where
//     T: ?Sized,
// source
// impl<T> From<T> for T
// source
// impl<T, U> Into<U> for T
// where
//     U: From<T>,
// source
// impl<T, U> TryFrom<U> for T
// where
//     U: Into<T>,
// source
// impl<T, U> TryInto<U> for T
// where
//     U: TryFrom<T>,

// TCPSTREAM DOCUMENTATION

// Struct std::net::TcpStreamCopy item path
// 1.0.0 · source · [−]
// pub struct TcpStream(_);
// A TCP stream between a local and a remote socket.

// After creating a TcpStream by either connecting to a remote host or accepting a connection on a TcpListener, data can be transmitted by reading and writing to it.

// The connection will be closed when the value is dropped. The reading and writing portions of the connection can also be shut down individually with the shutdown method.

// The Transmission Control Protocol is specified in IETF RFC 793.

// Examples
// use std::io::prelude::*;
// use std::net::TcpStream;

// fn main() -> std::io::Result<()> {
//     let mut stream = TcpStream::connect("127.0.0.1:34254")?;

//     stream.write(&[1])?;
//     stream.read(&mut [0; 128])?;
//     Ok(())
// } // the stream is closed here
// Implementations
// source
// impl TcpStream
// source
// pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<TcpStream>
// Opens a TCP connection to a remote host.

// addr is an address of the remote host. Anything which implements ToSocketAddrs trait can be supplied for the address; see this trait documentation for concrete examples.

// If addr yields multiple addresses, connect will be attempted with each of the addresses until a connection is successful. If none of the addresses result in a successful connection, the error returned from the last connection attempt (the last address) is returned.

// Examples
// Open a TCP connection to 127.0.0.1:8080:

// use std::net::TcpStream;

// if let Ok(stream) = TcpStream::connect("127.0.0.1:8080") {
//     println!("Connected to the server!");
// } else {
//     println!("Couldn't connect to server...");
// }
// Open a TCP connection to 127.0.0.1:8080. If the connection fails, open a TCP connection to 127.0.0.1:8081:

// use std::net::{SocketAddr, TcpStream};

// let addrs = [
//     SocketAddr::from(([127, 0, 0, 1], 8080)),
//     SocketAddr::from(([127, 0, 0, 1], 8081)),
// ];
// if let Ok(stream) = TcpStream::connect(&addrs[..]) {
//     println!("Connected to the server!");
// } else {
//     println!("Couldn't connect to server...");
// }
// 1.21.0 · source
// pub fn connect_timeout(addr: &SocketAddr, timeout: Duration) -> Result<TcpStream>
// Opens a TCP connection to a remote host with a timeout.

// Unlike connect, connect_timeout takes a single SocketAddr since timeout must be applied to individual addresses.

// It is an error to pass a zero Duration to this function.

// Unlike other methods on TcpStream, this does not correspond to a single system call. It instead calls connect in nonblocking mode and then uses an OS-specific mechanism to await the completion of the connection request.

// source
// pub fn peer_addr(&self) -> Result<SocketAddr>
// Returns the socket address of the remote peer of this TCP connection.

// Examples
// use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// assert_eq!(stream.peer_addr().unwrap(),
//            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)));
// source
// pub fn local_addr(&self) -> Result<SocketAddr>
// Returns the socket address of the local half of this TCP connection.

// Examples
// use std::net::{IpAddr, Ipv4Addr, TcpStream};

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// assert_eq!(stream.local_addr().unwrap().ip(),
//            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
// source
// pub fn shutdown(&self, how: Shutdown) -> Result<()>
// Shuts down the read, write, or both halves of this connection.

// This function will cause all pending and future I/O on the specified portions to return immediately with an appropriate value (see the documentation of Shutdown).

// Platform-specific behavior
// Calling this function multiple times may result in different behavior, depending on the operating system. On Linux, the second call will return Ok(()), but on macOS, it will return ErrorKind::NotConnected. This may change in the future.

// Examples
// use std::net::{Shutdown, TcpStream};

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// stream.shutdown(Shutdown::Both).expect("shutdown call failed");
// source
// pub fn try_clone(&self) -> Result<TcpStream>
// Creates a new independently owned handle to the underlying socket.

// The returned TcpStream is a reference to the same stream that this object references. Both handles will read and write the same stream of data, and options set on one stream will be propagated to the other stream.

// Examples
// use std::net::TcpStream;

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// let stream_clone = stream.try_clone().expect("clone failed...");
// 1.4.0 · source
// pub fn set_read_timeout(&self, dur: Option<Duration>) -> Result<()>
// Sets the read timeout to the timeout specified.

// If the value specified is None, then read calls will block indefinitely. An Err is returned if the zero Duration is passed to this method.

// Platform-specific behavior
// Platforms may return a different error code whenever a read times out as a result of setting this option. For example Unix typically returns an error of the kind WouldBlock, but Windows may return TimedOut.

// Examples
// use std::net::TcpStream;

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// stream.set_read_timeout(None).expect("set_read_timeout call failed");
// An Err is returned if the zero Duration is passed to this method:

// use std::io;
// use std::net::TcpStream;
// use std::time::Duration;

// let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
// let result = stream.set_read_timeout(Some(Duration::new(0, 0)));
// let err = result.unwrap_err();
// assert_eq!(err.kind(), io::ErrorKind::InvalidInput)
// 1.4.0 · source
// pub fn set_write_timeout(&self, dur: Option<Duration>) -> Result<()>
// Sets the write timeout to the timeout specified.

// If the value specified is None, then write calls will block indefinitely. An Err is returned if the zero Duration is passed to this method.

// Platform-specific behavior
// Platforms may return a different error code whenever a write times out as a result of setting this option. For example Unix typically returns an error of the kind WouldBlock, but Windows may return TimedOut.

// Examples
// use std::net::TcpStream;

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// stream.set_write_timeout(None).expect("set_write_timeout call failed");
// An Err is returned if the zero Duration is passed to this method:

// use std::io;
// use std::net::TcpStream;
// use std::time::Duration;

// let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
// let result = stream.set_write_timeout(Some(Duration::new(0, 0)));
// let err = result.unwrap_err();
// assert_eq!(err.kind(), io::ErrorKind::InvalidInput)
// 1.4.0 · source
// pub fn read_timeout(&self) -> Result<Option<Duration>>
// Returns the read timeout of this socket.

// If the timeout is None, then read calls will block indefinitely.

// Platform-specific behavior
// Some platforms do not provide access to the current timeout.

// Examples
// use std::net::TcpStream;

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// stream.set_read_timeout(None).expect("set_read_timeout call failed");
// assert_eq!(stream.read_timeout().unwrap(), None);
// 1.4.0 · source
// pub fn write_timeout(&self) -> Result<Option<Duration>>
// Returns the write timeout of this socket.

// If the timeout is None, then write calls will block indefinitely.

// Platform-specific behavior
// Some platforms do not provide access to the current timeout.

// Examples
// use std::net::TcpStream;

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// stream.set_write_timeout(None).expect("set_write_timeout call failed");
// assert_eq!(stream.write_timeout().unwrap(), None);
// 1.18.0 · source
// pub fn peek(&self, buf: &mut [u8]) -> Result<usize>
// Receives data on the socket from the remote address to which it is connected, without removing that data from the queue. On success, returns the number of bytes peeked.

// Successive calls return the same data. This is accomplished by passing MSG_PEEK as a flag to the underlying recv system call.

// Examples
// use std::net::TcpStream;

// let stream = TcpStream::connect("127.0.0.1:8000")
//                        .expect("Couldn't connect to the server...");
// let mut buf = [0; 10];
// let len = stream.peek(&mut buf).expect("peek failed");
// source
// pub fn set_linger(&self, linger: Option<Duration>) -> Result<()>
// 🔬
//  This is a nightly-only experimental API. (
// tcp_linger

// #88494
// )
// Sets the value of the SO_LINGER option on this socket.

// This value controls how the socket is closed when data remains to be sent. If SO_LINGER is set, the socket will remain open for the specified duration as the system attempts to send pending data. Otherwise, the system may close the socket immediately, or wait for a default timeout.

// Examples
// #![feature(tcp_linger)]

// use std::net::TcpStream;
// use std::time::Duration;

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// stream.set_linger(Some(Duration::from_secs(0))).expect("set_linger call failed");
// source
// pub fn linger(&self) -> Result<Option<Duration>>
// 🔬
//  This is a nightly-only experimental API. (
// tcp_linger

// #88494
// )
// Gets the value of the SO_LINGER option on this socket.

// For more information about this option, see TcpStream::set_linger.

// Examples
// #![feature(tcp_linger)]

// use std::net::TcpStream;
// use std::time::Duration;

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// stream.set_linger(Some(Duration::from_secs(0))).expect("set_linger call failed");
// assert_eq!(stream.linger().unwrap(), Some(Duration::from_secs(0)));
// 1.9.0 · source
// pub fn set_nodelay(&self, nodelay: bool) -> Result<()>
// Sets the value of the TCP_NODELAY option on this socket.

// If set, this option disables the Nagle algorithm. This means that segments are always sent as soon as possible, even if there is only a small amount of data. When not set, data is buffered until there is a sufficient amount to send out, thereby avoiding the frequent sending of small packets.

// Examples
// use std::net::TcpStream;

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// stream.set_nodelay(true).expect("set_nodelay call failed");
// 1.9.0 · source
// pub fn nodelay(&self) -> Result<bool>
// Gets the value of the TCP_NODELAY option on this socket.

// For more information about this option, see TcpStream::set_nodelay.

// Examples
// use std::net::TcpStream;

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// stream.set_nodelay(true).expect("set_nodelay call failed");
// assert_eq!(stream.nodelay().unwrap_or(false), true);
// 1.9.0 · source
// pub fn set_ttl(&self, ttl: u32) -> Result<()>
// Sets the value for the IP_TTL option on this socket.

// This value sets the time-to-live field that is used in every packet sent from this socket.

// Examples
// use std::net::TcpStream;

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// stream.set_ttl(100).expect("set_ttl call failed");
// 1.9.0 · source
// pub fn ttl(&self) -> Result<u32>
// Gets the value of the IP_TTL option for this socket.

// For more information about this option, see TcpStream::set_ttl.

// Examples
// use std::net::TcpStream;

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// stream.set_ttl(100).expect("set_ttl call failed");
// assert_eq!(stream.ttl().unwrap_or(0), 100);
// 1.9.0 · source
// pub fn take_error(&self) -> Result<Option<Error>>
// Gets the value of the SO_ERROR option on this socket.

// This will retrieve the stored error in the underlying socket, clearing the field in the process. This can be useful for checking errors between calls.

// Examples
// use std::net::TcpStream;

// let stream = TcpStream::connect("127.0.0.1:8080")
//                        .expect("Couldn't connect to the server...");
// stream.take_error().expect("No error was expected...");
// 1.9.0 · source
// pub fn set_nonblocking(&self, nonblocking: bool) -> Result<()>
// Moves this TCP stream into or out of nonblocking mode.

// This will result in read, write, recv and send operations becoming nonblocking, i.e., immediately returning from their calls. If the IO operation is successful, Ok is returned and no further action is required. If the IO operation could not be completed and needs to be retried, an error with kind io::ErrorKind::WouldBlock is returned.

// On Unix platforms, calling this method corresponds to calling fcntl FIONBIO. On Windows calling this method corresponds to calling ioctlsocket FIONBIO.

// Examples
// Reading bytes from a TCP stream in non-blocking mode:

// use std::io::{self, Read};
// use std::net::TcpStream;

// let mut stream = TcpStream::connect("127.0.0.1:7878")
//     .expect("Couldn't connect to the server...");
// stream.set_nonblocking(true).expect("set_nonblocking call failed");

// let mut buf = vec![];
// loop {
//     match stream.read_to_end(&mut buf) {
//         Ok(_) => break,
//         Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
//             // wait until network socket is ready, typically implemented
//             // via platform-specific APIs such as epoll or IOCP
//             wait_for_fd();
//         }
//         Err(e) => panic!("encountered IO error: {e}"),
//     };
// };
// println!("bytes: {buf:?}");
// Trait Implementations
// 1.63.0 · source
// impl AsFd for TcpStream
// source
// fn as_fd(&self) -> BorrowedFd<'_>
// Available on
// Unix
//  only.
// Borrows the file descriptor. Read more

// source
// impl AsRawFd for TcpStream
// source
// fn as_raw_fd(&self) -> RawFd
// Available on
// Unix
//  only.
// Extracts the raw file descriptor. Read more

// source
// impl AsRawSocket for TcpStream
// Available on
// Windows
//  only.
// source
// fn as_raw_socket(&self) -> RawSocket
// Extracts the raw socket. Read more

// 1.63.0 · source
// impl AsSocket for TcpStream
// Available on
// Windows
//  only.
// source
// fn as_socket(&self) -> BorrowedSocket<'_>
// Borrows the socket.

// source
// impl Debug for TcpStream
// source
// fn fmt(&self, f: &mut Formatter<'_>) -> Result
// Formats the value using the given formatter. Read more

// 1.63.0 · source
// impl From<OwnedFd> for TcpStream
// source
// fn from(owned_fd: OwnedFd) -> Self
// Converts to this type from the input type.

// 1.63.0 · source
// impl From<OwnedSocket> for TcpStream
// Available on
// Windows
//  only.
// source
// fn from(owned: OwnedSocket) -> Self
// Converts to this type from the input type.

// 1.63.0 · source
// impl From<TcpStream> for OwnedFd
// source
// fn from(tcp_stream: TcpStream) -> OwnedFd
// Converts to this type from the input type.

// 1.63.0 · source
// impl From<TcpStream> for OwnedSocket
// Available on
// Windows
//  only.
// source
// fn from(tcp_stream: TcpStream) -> OwnedSocket
// Converts to this type from the input type.

// 1.1.0 · source
// impl FromRawFd for TcpStream
// source
// unsafe fn from_raw_fd(fd: RawFd) -> TcpStreamⓘ
// Available on
// Unix
//  only.
// Constructs a new instance of Self from the given raw file descriptor. Read more

// 1.1.0 · source
// impl FromRawSocket for TcpStream
// Available on
// Windows
//  only.
// source
// unsafe fn from_raw_socket(sock: RawSocket) -> TcpStreamⓘ
// Constructs a new I/O object from the specified raw socket. Read more

// 1.4.0 · source
// impl IntoRawFd for TcpStream
// source
// fn into_raw_fd(self) -> RawFd
// Available on
// Unix
//  only.
// Consumes this object, returning the raw underlying file descriptor. Read more

// 1.4.0 · source
// impl IntoRawSocket for TcpStream
// Available on
// Windows
//  only.
// source
// fn into_raw_socket(self) -> RawSocket
// Consumes this object, returning the raw underlying socket. Read more

// source
// impl Read for &TcpStream
// source
// fn read(&mut self, buf: &mut [u8]) -> Result<usize>
// Pull some bytes from this source into the specified buffer, returning how many bytes were read. Read more

// source
// fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize>
// Like read, except that it reads into a slice of buffers. Read more

// source
// fn is_read_vectored(&self) -> bool
// 🔬
//  This is a nightly-only experimental API. (
// can_vector

// #69941
// )
// Determines if this Reader has an efficient read_vectored implementation. Read more

// source
// fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>
// Read all bytes until EOF in this source, placing them into buf. Read more

// source
// fn read_to_string(&mut self, buf: &mut String) -> Result<usize>
// Read all bytes until EOF in this source, appending them to buf. Read more

// 1.6.0 · source
// fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>
// Read the exact number of bytes required to fill buf. Read more

// source
// fn read_buf(&mut self, buf: &mut ReadBuf<'_>) -> Result<()>
// 🔬
//  This is a nightly-only experimental API. (
// read_buf

// #78485
// )
// Pull some bytes from this source into the specified buffer. Read more

// source
// fn read_buf_exact(&mut self, buf: &mut ReadBuf<'_>) -> Result<()>
// 🔬
//  This is a nightly-only experimental API. (
// read_buf

// #78485
// )
// Read the exact number of bytes required to fill buf. Read more

// source
// fn by_ref(&mut self) -> &mut Self
// where
//     Self: Sized,
// Creates a “by reference” adaptor for this instance of Read. Read more

// source
// fn bytes(self) -> Bytes<Self>ⓘ
// where
//     Self: Sized,
// Transforms this Read instance to an Iterator over its bytes. Read more

// source
// fn chain<R: Read>(self, next: R) -> Chain<Self, R>ⓘ
// where
//     Self: Sized,
// Creates an adapter which will chain this stream with another. Read more

// source
// fn take(self, limit: u64) -> Take<Self>ⓘ
// where
//     Self: Sized,
// Creates an adapter which will read at most limit bytes from it. Read more

// source
// impl Read for TcpStream
// source
// fn read(&mut self, buf: &mut [u8]) -> Result<usize>
// Pull some bytes from this source into the specified buffer, returning how many bytes were read. Read more

// source
// fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize>
// Like read, except that it reads into a slice of buffers. Read more

// source
// fn is_read_vectored(&self) -> bool
// 🔬
//  This is a nightly-only experimental API. (
// can_vector

// #69941
// )
// Determines if this Reader has an efficient read_vectored implementation. Read more

// source
// fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>
// Read all bytes until EOF in this source, placing them into buf. Read more

// source
// fn read_to_string(&mut self, buf: &mut String) -> Result<usize>
// Read all bytes until EOF in this source, appending them to buf. Read more

// 1.6.0 · source
// fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>
// Read the exact number of bytes required to fill buf. Read more

// source
// fn read_buf(&mut self, buf: &mut ReadBuf<'_>) -> Result<()>
// 🔬
//  This is a nightly-only experimental API. (
// read_buf

// #78485
// )
// Pull some bytes from this source into the specified buffer. Read more

// source
// fn read_buf_exact(&mut self, buf: &mut ReadBuf<'_>) -> Result<()>
// 🔬
//  This is a nightly-only experimental API. (
// read_buf

// #78485
// )
// Read the exact number of bytes required to fill buf. Read more

// source
// fn by_ref(&mut self) -> &mut Self
// where
//     Self: Sized,
// Creates a “by reference” adaptor for this instance of Read. Read more

// source
// fn bytes(self) -> Bytes<Self>ⓘ
// where
//     Self: Sized,
// Transforms this Read instance to an Iterator over its bytes. Read more

// source
// fn chain<R: Read>(self, next: R) -> Chain<Self, R>ⓘ
// where
//     Self: Sized,
// Creates an adapter which will chain this stream with another. Read more

// source
// fn take(self, limit: u64) -> Take<Self>ⓘ
// where
//     Self: Sized,
// Creates an adapter which will read at most limit bytes from it. Read more

// source
// impl Write for &TcpStream
// source
// fn write(&mut self, buf: &[u8]) -> Result<usize>
// Write a buffer into this writer, returning how many bytes were written. Read more

// source
// fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize>
// Like write, except that it writes from a slice of buffers. Read more

// source
// fn is_write_vectored(&self) -> bool
// 🔬
//  This is a nightly-only experimental API. (
// can_vector

// #69941
// )
// Determines if this Writer has an efficient write_vectored implementation. Read more

// source
// fn flush(&mut self) -> Result<()>
// Flush this output stream, ensuring that all intermediately buffered contents reach their destination. Read more

// source
// fn write_all(&mut self, buf: &[u8]) -> Result<()>
// Attempts to write an entire buffer into this writer. Read more

// source
// fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> Result<()>
// 🔬
//  This is a nightly-only experimental API. (
// write_all_vectored

// #70436
// )
// Attempts to write multiple buffers into this writer. Read more

// source
// fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<()>
// Writes a formatted string into this writer, returning any error encountered. Read more

// source
// fn by_ref(&mut self) -> &mut Self
// where
//     Self: Sized,
// Creates a “by reference” adapter for this instance of Write. Read more

// source
// impl Write for TcpStream
// source
// fn write(&mut self, buf: &[u8]) -> Result<usize>
// Write a buffer into this writer, returning how many bytes were written. Read more

// source
// fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize>
// Like write, except that it writes from a slice of buffers. Read more

// source
// fn is_write_vectored(&self) -> bool
// 🔬
//  This is a nightly-only experimental API. (
// can_vector

// #69941
// )
// Determines if this Writer has an efficient write_vectored implementation. Read more

// source
// fn flush(&mut self) -> Result<()>
// Flush this output stream, ensuring that all intermediately buffered contents reach their destination. Read more

// source
// fn write_all(&mut self, buf: &[u8]) -> Result<()>
// Attempts to write an entire buffer into this writer. Read more

// source
// fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> Result<()>
// 🔬
//  This is a nightly-only experimental API. (
// write_all_vectored

// #70436
// )
// Attempts to write multiple buffers into this writer. Read more

// source
// fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<()>
// Writes a formatted string into this writer, returning any error encountered. Read more

// source
// fn by_ref(&mut self) -> &mut Self
// where
//     Self: Sized,
// Creates a “by reference” adapter for this instance of Write. Read more

// Auto Trait Implementations
// impl RefUnwindSafe for TcpStream
// impl Send for TcpStream
// impl Sync for TcpStream
// impl Unpin for TcpStream
// impl UnwindSafe for TcpStream
// Blanket Implementations
// source
// impl<T> Any for T
// where
//     T: 'static + ?Sized,
// source
// impl<T> Borrow<T> for T
// where
//     T: ?Sized,
// source
// impl<T> BorrowMut<T> for T
// where
//     T: ?Sized,
// source
// impl<T> From<T> for T
// source
// impl<T, U> Into<U> for T
// where
//     U: From<T>,
// source
// impl<T, U> TryFrom<U> for T
// where
//     U: Into<T>,
// source
// impl<T, U> TryInto<U> for T
// where
//     U: TryFrom<T>,

// instructions:
// Group Project 1: Warm up of Interprocess Communication
// CECS 326 – Operating Systems
// You should submit the required deliverable materials on BeachBoard by 11:55pm, September
// 29th (Thursday), 2022. We provide sample code files that you can complete the required functions.
// 1. Description
// Question 1:
// Sockets lecture describes certain port numbers as being well known—that is, they provide standard
// services. Port 17 is known as the quote-of-the-day service. When a client connects to port 17 on a
// server, the server responds with a quote for that day.
// Modify the date server shown in Figure 1 so that it delivers a quote of the day rather than the current
// date. The quotes should be printable ASCII characters and should contain fewer than 512 characters,
// although multiple lines are allowed. Since these well-known ports are reserved and therefore
// unavailable, have your server listen to port 6017. The date client can be used to read the quotes
// returned by your server.
// Figure 1. Data Server

// 3: The Required Deliverable Materials
// (1) A README file, which describes how we can compile and run your code.
// (2) Your source code, you may use a Makefile (for C) and be submitted in the required format.
// (3) Your short report, which discusses the design of your program.
// (4) A recorded video shows the output and runtime
// 3. Submission Requirements
// You need to strictly follow the instructions listed below:
// 1) This is a group project, please submit a .zip/.rar file that contains all files, only one submission
// from one group.
// 2) Make a video to record your code execution and outputs. The video should present your name
// or time as identification (You are suggested to upload the video to YouTube and put the link into
// your report).
// 3) The submission should include your source code and project report. Do not submit your binary
// code. Project report should contain your groupmates name and ID.
// 4) Your code must be able to compile; otherwise, you will receive a grade of zero.
// 5) Your code should not produce anything else other than the required information in the output
// file.
// 7) If you code is partially completed, please explain the details in the report what has been
// completed and the status of the missing parts, we will grade it based on the entire performance.
// 8) Provide sufficient comments in your code to help the TA understand your code. This is
// important for you to get at least partial credit in case your submitted code does not work properly.

// import rand and thread_rng
use rand::prelude::*;

use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

static QUOTES: [&str; 14] = [
    "The greatest glory in living lies not in never falling, but in rising every time we fall.",
    "The way to get started is to quit talking and begin doing.",
    "Your time is limited, so don’t waste it living someone else’s life.",
    "If life were predictable it would cease to be life, and be without flavor.",
    "If you set your goals ridiculously high and it’s a failure, you will fail above everyone else’s success.",
    "The best way to predict the future is to invent it. - Alan Kay",
    "A computer lets you make more mistakes faster than any invention in human history - with the possible exceptions of handguns and tequila. - Mitch Ratcliffe",
    "The computer was born to solve problems that did not exist before. - Bill Gates",
    "Our most powerful 21st-century technologies – robotics, genetic engineering, and nanotech – are threatening to make humans an endangered species. - Vernor Vinge",
    "The most exciting phrase to hear in science, the one that heralds new discoveries, is not 'Eureka!' but 'That's funny...' - Isaac Asimov",
    "One of the most remarkable things about the universe is that it is comprehensible. - Stephen Hawking",
    "Reality is merely an illusion, albeit a very persistent one. - Albert Einstein",
    "The most incomprehensible thing about the world is that it is at all comprehensible. - Albert Einstein",
    "The most beautiful experience we can have is the mysterious. It is the fundamental emotion that stands at the cradle of true art and true science. - Albert Einstein"
];
// this function should return a random quote from the QUOTES array via a quote client and qutoe server

fn main() {
    // create a TCP listener
    let listener = TcpListener::bind("127.0.0.1:6017").unwrap();
    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    
}






struct QuoteServer {
    listener: TcpListener,
}

impl QuoteServer {
    fn listen(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        QuoteServer::handle_client(stream);
                    });
                }
                Err(e) => {
                    // connection failed
                    println!("Error: {}", e);
                }
            }
        }
    }
    fn handle_client(mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let quote = QUOTES[thread_rng().gen_range(0..14)];
        stream.write(quote.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    fn new(addr: &str) -> Result<QuoteServer, Error> {
        let listener = TcpListener::bind(addr)?;
        Ok(QuoteServer { listener })
    }
}

struct QuoteClient {
    stream: TcpStream,
}

impl QuoteClient {
    fn listen(&mut self) {
        let mut buffer = [0; 512];

        self.stream.read(&mut buffer).unwrap();
        println!("test");
        let quote = String::from_utf8_lossy(&buffer[..]);
        println!("Received quote: {}", quote);
    }
    fn new(addr: &str) -> Result<QuoteClient, Error> {
        let stream = TcpStream::connect(addr)?;
        println!("Connected to the server inside Client!");
        Ok(QuoteClient { stream })
    }
}