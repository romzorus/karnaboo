use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

mod commandline;
mod localsystem;
mod networking;

fn main() {
/* Global workflow :
    1. parse the request
    2. get local system information
    3. verify request coherence according to system information
    4. send request
    5. wait for reception confirmation
    6. wait for decision and technical informations
    7. implement the decision with technical informations
*/
    // *********** 1. parse the request
    let args: Vec<String> = env::args().collect();
    let (role, server_socket) = commandline::command_line_parsing(args);

    println!("Server : {:?}", server_socket);
    println!("Role : {}", role);


    // println!("Request summary :");
    // println!("Server info : {}", server);
    // println!("Requested role : {}", role);

    // *********** 2. get local system information
    // localsystem::get_local_system_info();

    // *********** 3. verify request consistency with the local system
    // *********** 4. send request

    // Building request content
    // let content = NodeClient {
    //     ip: String::from("10.99.99.99"),
    //     hostname: String::from("zoulou-PC"),
    // };
    // let request_content = NodeHostRequest::Client(content);

    // Sending request to server
    // println!("Sending request to server.");
    // println!("Request content : {:?}", request_content);
    // send_request(request_content);

    // *********** 5. wait for reception confirmation
    // *********** 6. wait for decision and technical informations
    // *********** 7. implement the decision with technical informations

    
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