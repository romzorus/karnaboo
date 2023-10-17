use serde::Serialize;
use std::net::{TcpStream, TcpListener, SocketAddr};
use std::io::{Read, Write};
use std::process::Output;

use crate::{FinalInstructions, ExecutionResult};
use crate::localsystem::LocalSystemConfig;

pub fn send_request(server_socket: SocketAddr, role: String, local_conf: LocalSystemConfig) {
    
    // Building content
    let content: NodeHostRequest;

    if role == "client" {
        content = NodeHostRequest::Client(NodeClient {
            hostname: local_conf.hostname,
            ip: String::from("0.0.0.0"),
            osname: local_conf.osname,
            osversion: local_conf.osversion,
            _key: local_conf._key
        });
    } else if role == "diss" {
        content = NodeHostRequest::Diss(NodeDiss {
            hostname: local_conf.hostname,
            ip: String::from("0.0.0.0"),
            osname: local_conf.osname,
            osversion: local_conf.osversion,
            _key: local_conf._key
        });
    } else {
        content = NodeHostRequest::Reps(NodeReps {
            hostname: local_conf.hostname,
            ip: String::from("0.0.0.0"),
            osname: local_conf.osname,
            osversion: local_conf.osversion,
            _key: local_conf._key
        });
    }

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

pub fn get_instructions_from_server() -> FinalInstructions {
    let listener = TcpListener::bind("0.0.0.0:9017").expect("Unable to open socket 0.0.0.0:9017");
    let (mut srv_stream, _srv_socket) = listener.accept().expect("Unable to establish connexion");

    let mut buffer: [u8; 2048] = [0; 2048];
    let size = srv_stream
        .read(&mut buffer)
        .expect("Unable to read from TcpStream");
    let serialized_content = String::from_utf8_lossy(&buffer[..size]);
    let final_instructions: FinalInstructions =
        serde_json::from_str(&serialized_content)
            .expect("Unable to deserialize data received from TcpStream");
    final_instructions
}

pub fn send_exec_result_to_server(script_output: Output, server_socket: SocketAddr) {
    // std::process::Output type doesn't natively support serialization.
    // [Until I find a better solution], another type ExecutionResult is defined
    // and used to serialize and send the result
    let execution_result = ExecutionResult {
        exit_status: script_output.status.to_string(),
        stdout: String::from_utf8_lossy(&script_output.stdout[..]).to_string(),
        stderr: String::from_utf8_lossy(&script_output.stderr[..]).to_string()
    };

    // Serialization before sending to socket
    let serialized_script_output = serde_json::to_string(&execution_result).unwrap();

    // Open a socket
    let mut socket_srv =
        TcpStream::connect(format!("{}:9016", server_socket.ip())).expect("Unable to connect to Karnaboo server");
    
    // Send to server the serialized request
    socket_srv
        .write(&serialized_script_output.as_bytes())
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
    pub _key: String,
}
#[derive(Debug, Serialize)]
pub struct NodeReps {
    pub hostname: String,
    pub ip: String,
    pub osname: String,
    pub osversion: String,
    pub _key: String,
}
#[derive(Debug, Serialize)]
pub struct NodeDiss {
    pub hostname: String,
    pub ip: String,
    pub osname: String,
    pub osversion: String,
    pub _key: String,
}
#[derive(Debug, Serialize)]
pub struct NodeOs {
    pub name: String,
    pub version: String,
}
