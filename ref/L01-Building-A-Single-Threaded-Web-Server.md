**We'll start by getting a single-threaded web server working.**

## 0. Web Server Protocols, A Brief Overview

Before we begin, let's look at a quick overview of the protocols involved in building web servers. The details of these protocols are beyond the scope of this book, but a brief overview will give you the information you need.

_The two main protocols involved in web servers_ are:

- Hypertext Transfer Protocol (HTTP), and
- Transmission Control Protocol (TCP).

_Both protocols are request-response protocols_, meaning a client initiates requests and a server listens to the requests and provides a response to the client. The contents of those requests and responses are defined by the protocols.

**TCP is the lower-level protocol that describes the details of** _how information gets from one server to another but doesn't specify what that information is_. **HTTP builds on top of TCP** _by defining the contents of the requests and responses_. It's technically possible to use HTTP with other protocols, but in the vast majority of cases, HTTP sends its data over TCP.

We'll work with the **raw bytes of TCP and HTTP requests and responses**.

## 1. Listening to the TCP Connection

*Our web server needs to listen to a TCP connection, so that's the first part we'll work on*. The standard library offers a `std::net` module that lets us do this. Let's make a new project in the usual fashion:

```sh
$ cargo new webserver
     Created binary (application) `webserver` project
$ cd webserver
```

Now put the following code in src/main.rs to start. Using `TcpListener` we can listen at the local address `127.0.0.1:7878` for incoming TCP streams. When it gets an incoming stream, it will print `Connection established!`.

Filename: src/main.rs

```rust
// Listening for incoming streams and printing a message when we receive a stream
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
```

In the address, the section before the colon is an IP address representing your computer (this is the same on every computer and doesn't represent the authors' computer specifically), and `7878` is the port. We've chosen this port for two reasons: HTTP isn't normally accepted on this port so our server is unlikely to conflict with any other web server you might have running on your machine.

The `bind` function in this scenario works like the new function in that it will return a new `TcpListener` instance. The function is called `bind` because, in networking, connecting to a port to listen to is known as "binding to a port."

The `bind` function returns a `Result<T, E>`, which indicates that it's possible for binding to fail. For example, *connecting to port 80 requires administrator privileges* (nonadministrators can listen only on ports higher than 1023), *so if we tried to connect to port 80 without being an administrator, binding wouldn't work*. Binding also wouldn't work, for example, if we ran two instances of our program and so had two programs listening to the same port. Because we're writing a basic server just for learning purposes, we won't worry about handling these kinds of errors; instead, we use unwrap to stop the program if errors happen.

The `incoming` method on `TcpListener` *returns an iterator that gives us a sequence of streams* (more specifically, streams of type `TcpStream`). A single stream represents an open connection between the client and the server. **A connection is the name for the full request and response process in which:**

- a client connects to the server,
- the server generates a response, and
- the server closes the connection.

As such, we will read from the `TcpStream` to see what the client sent and then write our response to the stream to send data back to the client. Overall, this for loop will process each connection in turn and produce a series of streams for us to handle.

For now, our handling of the stream consists of calling `unwrap` to terminate our program if the stream has any errors; if there aren't any errors, the program prints a message. We'll add more functionality for the success case in the next listing. *The reason we might receive errors from the incoming method when a client connects to the server is that we're not actually iterating over connections*. **Instead, we're iterating over connection attempts**. The connection might not be successful for a number of reasons, many of them operating system specific. For example, many operating systems have a limit to the number of simultaneous open connections they can support; new connection attempts beyond that number will produce an error until some of the open connections are closed.

Let's try running this code! Invoke `cargo run` in the terminal and then load `127.0.0.1:7878` in a web browser. The browser should show an error message like "Connection reset," because the server isn't currently sending back any data. But when you look at your terminal, you should see several messages that were printed when the browser connected to the server!

     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!

Sometimes, you'll see multiple messages printed for one browser request; the reason might be that the browser is making a request for the page as well as a request for other resources, like the favicon.ico icon that appears in the browser tab.

It could also be that the browser is trying to connect to the server multiple times because the server isn't responding with any data. When stream goes out of scope and is dropped at the end of the loop, the connection is closed as part of the drop implementation. Browsers sometimes deal with closed connections by retrying, because the problem might be temporary. The important factor is that we've successfully gotten a handle to a TCP connection!

Remember to stop the program by pressing ctrl-c when you're done running a particular version of the code. Then restart the program by invoking the cargo run command after you've made each set of code changes to make sure you're running the newest code.
