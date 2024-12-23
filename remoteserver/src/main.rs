use std::error::Error;
use std::io::Read;
use std::net::TcpListener;

const SERVER_ADDR: &str = "127.0.0.1:4543";

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(SERVER_ADDR)?;

    for stream in listener.incoming() {
        let mut url = String::new();
        stream?.read_to_string(&mut url)?;

        println!("open url {url}");
        webbrowser::open(&url)?;
    }

    Ok(())
}
