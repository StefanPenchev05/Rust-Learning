mod lib;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use lib::HttpResponse;

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    println!("Server is listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection Established, {:?}", stream);

        if let Err(_) = handle_connection(stream) {
            panic!("An error occurd!")
        };
    }
}


fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        (HttpResponse::Ok.to_string(), "hello.html")
    } else {
        (HttpResponse::NotFound.to_string(), "NotFound.html")
    };


    let contents = fs::read_to_string(format!("./src/{}", filename)).unwrap();
    let length = contents.len();

    let response = HttpResponse::return_content(status_line, length, contents);

    println!("{:#?}", request_line);
    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}
