use serde::Serialize;
use std::io::Write;
use std::net::{TcpStream, SocketAddr};

pub fn send_request(content: NodeHostRequest, server_socket: SocketAddr) {
    // Serialization before sending to socket
    let serialized_request = serde_json::to_string(&content).unwrap();

    // Open a socket
    let mut socket_srv =
        TcpStream::connect(server_socket).expect("Unable to connect to Karnaboo server");

    // Send to server the serialized request
    socket_srv
        .write(&serialized_request.as_bytes())
        .expect("Unable to send data through socket");
}

// Defining data types and enums to build request
#[derive(Debug, Serialize)]
pub enum NodeHostRequest {
    Client(NodeClient),
    Diss(NodeDiss),
    Reps(NodeReps),
}
#[derive(Debug, Serialize)]
pub struct NodeClient {
    pub hostname: String,
    pub ip: String,
    pub osname: String,
    pub osversion: String,
    pub hostid: String,
}
#[derive(Debug, Serialize)]
pub struct NodeReps {
    pub hostname: String,
    pub ip: String,
    pub osname: String,
    pub osversion: String,
    pub hostid: String,
}
#[derive(Debug, Serialize)]
pub struct NodeDiss {
    pub hostname: String,
    pub ip: String,
    pub osname: String,
    pub osversion: String,
    pub hostid: String,
}
#[derive(Debug, Serialize)]
pub struct NodeOs {
    pub name: String,
    pub version: String,
}
