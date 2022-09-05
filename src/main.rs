use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
mod png_gen;

fn main() {
	let listener: TcpListener = TcpListener::bind("127.0.0.1:8000").unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		println!("Connection established!");

		handle_connection(stream);
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 1024];

	stream.read(&mut buffer).unwrap();

	println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

	let response = format!(
		"HTTP:1.1 200 OK\r\n{}\r\n\r\n",
		"Content-type: image/png"
	);
	let response_content = png_gen::default_png_gen();
	println!("Headers as bytes: {:?}", response.as_bytes());
	println!("Answer png made: {:?}", response_content);

	stream.write(&[response.as_bytes(), &response_content[..]].concat()).unwrap();
	stream.flush().unwrap();
}