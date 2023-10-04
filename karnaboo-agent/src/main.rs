use serde::{Deserialize, Serialize};
use std::io::Write;
use std::net::TcpStream;

fn main() {
    // Building request content
    let content = NodeClient {
        ip: String::from("10.99.99.99"),
        hostname: String::from("zoulou-PC"),
    };

    let request_content = NodeHostRequest::Client(content);

    // Sending request to server
    println!("Sending request to server.");
    println!("Request content : {:?}", request_content);
    send_request(request_content);
}

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

// Defining data types and enums to build request
#[derive(Debug, Serialize)]
enum NodeHostRequest {
    Client(NodeClient),
    Diss(NodeDiss),
    Reps(NodeReps),
}
#[derive(Debug, Serialize)]
pub struct NodeClient {
    pub hostname: String,
    pub ip: String,
}
#[derive(Debug, Serialize)]
pub struct NodeReps {
    pub hostname: String,
    pub ip: String,
}
#[derive(Debug, Serialize)]
pub struct NodeDiss {
    pub hostname: String,
    pub ip: String,
}
#[derive(Debug, Serialize)]
pub struct NodeOs {
    pub name: String,
    pub version: String,
}

