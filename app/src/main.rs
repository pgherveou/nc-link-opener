use std::{env::args, io::Read, process::Command};

use url::{ParseError, Url};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(port) = args().nth(1) else {
        panic!("Usage: nc-link-opener <port>");
    };

    let port: u16 = port.parse()?;

    let listener = std::net::TcpListener::bind(format!("localhost:{}", port))?;
    println!("nc-link-opener Listening on port {}", port);
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        match url_from_request(stream) {
            Ok(url) => open_link(url),
            Err(err) => eprintln!("Failed to parse URL: {err:?}"),
        }
    }

    Ok(())
}

// Open the URL in the default browser
fn open_link(url: Url) {
    Command::new("open").arg(url.to_string()).spawn().unwrap();
}

// Parse the incoming request and return the URL
fn url_from_request(mut req: impl Read) -> Result<Url, ParseError> {
    let mut buffer = [0; 1024];
    let mut bytes_read = 0;

    loop {
        let read = req.read(&mut buffer).unwrap();
        bytes_read += read;
        if read == 0 {
            break;
        }
    }

    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
    Url::parse(&message)
}

#[test]
fn it_works() {
    let url = url_from_request("http://localhost:8080".as_bytes());
    assert_eq!(url, Url::parse("http://localhost:8080"));
}
