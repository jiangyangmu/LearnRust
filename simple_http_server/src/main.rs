use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:12345").unwrap();
	let mut id : i32 = 0;

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		let request_id = id;
		id += 1;

		thread::spawn(move || {
			println!("request {} begin", request_id);
			handle_connection(stream);
			println!("request {} end", request_id);
		});
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 1024];

	stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
	if !stream.read(&mut buffer).is_ok() {
		return;
	}

	let get = b"GET / HTTP/1.1\r\n";

	let (status_line, filename) = if buffer.starts_with(get) {
		("HTTP/1.1 200 OK\r\n\r\n", "index.html")
	} else {
		("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
	};

	let contents = fs::read_to_string(filename).unwrap();

	let response = format!("{}{}", status_line, contents);

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}