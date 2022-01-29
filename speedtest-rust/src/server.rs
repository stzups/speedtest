use std::io::{Read, Write};

pub fn listen<A: std::net::ToSocketAddrs>(address: A) -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind(address)?;
    eprintln!("listening on {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        let mut stream = stream?;
        let address = stream.peer_addr().unwrap().ip();
        println!("{}", address);
        let protocol = &mut [0u8; 1];
        stream.read_exact(protocol)?;
        stream.write(&match protocol[0] {
            71 => { get_http(address) } //71 represents ASCII letter G which is sent from an HTTP GET request
            _ => { return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, concat!("bad protocol: must be ", env!("CARGO_PKG_NAME"), " or HTTP GET"))); }
        })?;
    }
    Ok(())
}

fn get_http(address: std::net::IpAddr) -> Vec<u8> {
    let mut headers: Vec<u8> = Vec::new();
    let mut body: Vec<u8> = Vec::new();

    body.extend_from_slice(match address {
        std::net::IpAddr::V4(ip) => {
            ip.to_string()
        },
        std::net::IpAddr::V6(ip) => {
            ip.to_string()
        }
    }.as_bytes());

    headers.extend_from_slice("HTTP/1.1 200 OK\r\nContent-Type: text/plain;\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: ".as_bytes());
    headers.extend_from_slice(body.len().to_string().as_bytes());
    headers.extend_from_slice("\r\n\r\n".as_bytes());

    let mut response = headers;
    response.extend_from_slice(&body);

    return response;
}
