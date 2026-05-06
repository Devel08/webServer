use std::fs;
use std::io::{self, Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buff: [u8; 2048] = [0; 2048];

    let mut request = String::new();

    // Find requested page and give it to the user
    let _ = stream.read(&mut buff[..]).unwrap();

    let mut start_of_file: bool = false;
    for i in buff.iter().take(100).skip(1) {
        if *i == b'\n' {
            break;
        }
        if *i == b'/' && !start_of_file {
            start_of_file = true;
            continue;
        }
        if start_of_file && *i == b' ' {
            break;
        }
        if start_of_file {
            request.push(*i as char);
        }
    }

    println!("{}", request);
    if request.is_empty() {
        request.push_str("index.html");
    }
    let path = Path::new(&request);

    if path.extension().unwrap() == "html" {
        let status_line = "HTTP/1.1 200 OK\r\n";
        let headers = "Content-Type: text/html\r\nContent-Length: ";

        let content = fs::read_to_string(&request)?;
        let content_length = content.len();

        let response =
            format!("{}{}{}{}", status_line, headers, content_length, "\r\n\r\n") + &content;

        stream.write_all(response.as_bytes())?;

        println!("Sent data to the client.");
    } else {
        let status_line = "HTTP/1.1 200 OK\r\n";
        let mut headers = "";

        let cache_control = "Cache-Control: public, max-age=86400\r\n";

        if path.extension().unwrap() == "ico" {
            headers = "Content-Type: image/vnd.microsoft.icon\r\n";
        } else if path.extension().unwrap() == "gif" {
            headers = "Content-Type: image/gif\r\n"
        } else if path.extension().unwrap() == "ogg" {
            headers = "Content-Type: application/ogg\r\n"
        }

        let content = fs::read(&request)?;
        let content_length = content.len();

        let response = format!(
            "{}{}{}Content-Length: {}\r\n\r\n",
            status_line, headers, cache_control, content_length
        );

        stream.write_all(response.as_bytes())?;
        stream.write_all(&content)?;
    }

    println!("Sent data to the client.");

    Ok(())
}

fn main() -> io::Result<()> {
    println!("A server is listening on localhost:7878");

    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client {0} connected!", stream.local_addr().unwrap());

                handle_client(stream)?;
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
