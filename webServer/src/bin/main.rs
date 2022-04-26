use std::{fs, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;
use webServer::ThreadPool;

fn main() {
    println!("Hello, world!");

    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listner.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    };

}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let slow_get = b"GET /slow HTTP/1.1\r\n";

    let (first_line, file_name) = if buffer.starts_with(slow_get) {
        sleep(Duration::from_secs(6));
        ("HTTP/1.1 200 OK", "slow.html")

    } else {
        ("HTTP/1.1 200 OK", "index.html")
    };


    let content = fs::read_to_string(file_name).unwrap();

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
                           first_line,
                           content.len(),
                           content
    );

    stream.write(response.as_bytes()).unwrap();


    stream.flush().unwrap();
}