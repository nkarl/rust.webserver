use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use webserver::ThreadPool;

fn main() {
    let url = "localhost:7878";
    let listener = TcpListener::bind(url).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    /*
     * Request Form:
     *      Method Request-URI HTTP-Version CRLF
     *      headers CRLF
     *      message-body
     *
     * NOTE: GET request has no body
     */
    //println!("Request: {:#?}", http_request);

    /*
     * Validating the request and Selectively responding
     */
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    /*
     * Response Form:
     *      HTTP-Version Status-Code Reason-Phrase CRLF
     *      headers CRLF
     *      message-body
     */
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1"        => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1"   => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _                       => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
