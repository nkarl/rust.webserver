use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let url = "localhost:7878";
    let listener = TcpListener::bind(url).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    /*
     * Request Form:
     *      Method Request-URI HTTP-Version CRLF
     *      headers CRLF
     *      message-body
     *
     * NOTE: GET request has no body
     */
    println!("Request: {:#?}", http_request);

    /*
     * Response Form:
     *      HTTP-Version Status-Code Reason-Phrase CRLF
     *      headers CRLF
     *      message-body
     */
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
