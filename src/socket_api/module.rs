use std::{net::{TcpListener, TcpStream, UdpSocket, SocketAddr}, io::{Write, BufReader, BufRead}};

// browser call wtice. it happen call browser behavior .favicon icon get.
// block .favicon call.
pub fn call() {    

    println!("socket api module call");
    listen_client();
}

// https://doc.rust-lang.org/std/net/struct.TcpStream.html
fn listen_client() {

    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream)
    }

}

// read write 
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();

}

// refer: https://doc.rust-lang.org/std/net/struct.UdpSocket.html
fn udp_sock_sample() -> std::io::Result<()> {

    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 3400)),
        SocketAddr::from(([127, 0, 0, 1], 3401)),
    ];
    let socket = UdpSocket::bind(&addrs[..]).expect("couldn't bind to address");
    
    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf)?;

    // Redeclare `buf` as slice of the received data and send reverse data back to origin.
    // cuase big endian?
    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, &src)?;

    Ok(())
}
