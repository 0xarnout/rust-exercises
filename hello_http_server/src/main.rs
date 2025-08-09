use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    sync::{Arc, atomic::{AtomicBool, Ordering}},
};
use ctrlc;
use threading::ThreadPool;

fn main() {
    let exit = Arc::new(AtomicBool::new(false));
    let exit_copy = Arc::clone(&exit);
    // Warning, this handler will be called after Ctrl-C AND a request is received
    ctrlc::set_handler(move || {
        exit_copy.store(true, Ordering::Relaxed);
    }).expect("Failed to set up Ctrl-C handler");

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Couldn't setup listener");
    let pool = ThreadPool::new(10);

    let mut requests = listener.incoming();
    while !exit.load(Ordering::Relaxed) {
        let stream = requests.next().expect("Incoming Iterator should never yield None").unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}