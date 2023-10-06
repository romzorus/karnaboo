use std::net::TcpStream;
use std::io::Write;

use crate::NodeHostRequest;

fn send_request(request_content: NodeHostRequest) {
    // Serialization before sending to socket
    let serialized_request = serde_json::to_string(&request_content).unwrap();

    // Open a socket
    let addr_srv = "127.0.0.1:9015";
    let mut socket_srv =
        TcpStream::connect(addr_srv).expect("Unable to connect to Karnaboo server");

    // Send to server the serialized request
    socket_srv
        .write(&serialized_request.as_bytes())
        .expect("Unable to send data through socket");
}
