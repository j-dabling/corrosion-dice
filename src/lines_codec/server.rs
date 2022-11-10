use std::io;
use std::net::{IpAddr, Ipv4Addr, TcpStream, TcpListener};
use colored::Colorize;
use serde_json::Value;

use crate::cli::command_functions::dynamic_color;

// use crate:LinesCodec;
use crate::lines_codec::LinesCodec;

fn handle_connection(stream: TcpStream) -> io::Result<()> {
    // println!("THE IP ADDRESS IS: {}", &stream.peer_addr().unwrap());
    let peer_ip = &stream.peer_addr().unwrap().ip().clone();
    let mut codec = LinesCodec::new(stream)?;
    
    let message: String = codec.read_message()?;
    let object: Value = serde_json::from_str(&message[..]).unwrap();

    let result = object.get("roll").unwrap().as_u64();
    let n  = object.get("n").unwrap().as_u64();
    if peer_ip != &IpAddr::V4(Ipv4Addr::new(127,0,0,1)) {
        println!("{} {} {} {} {}                 ",
            peer_ip.to_string().green(),
            "rolled: ".green(),
            "[".white().bold(),
            dynamic_color(result.unwrap(), n.unwrap()),
            "]".white().bold());
    }
    
    Ok(())
}

pub fn start_corrosion_server(addir: Option<&str>) -> io::Result<()> {
    let address: &str = addir.unwrap_or("127.0.0.1:4000");
    
    eprintln!("Starting server on '{}'", address);

    let listener = TcpListener::bind(address)
        .expect("Was not able to start server.");
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            std::thread::spawn(move || {
                handle_connection(stream).map_err(|e| eprintln!("Error: {}", e))
            });
        }
    }
    Ok(())
}
