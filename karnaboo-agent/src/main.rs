use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::process::exit;

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
    let mut role = String::with_capacity(6); // "client | reps | diss"
    let mut server_socket: SocketAddr = "0.0.0.0:0".parse().unwrap();

    match commandline::command_line_parsing(args) {
        Ok((tmp_role, tmp_server_socket)) => {
            role = tmp_role;
            server_socket = tmp_server_socket;
        }
        Err(()) => {
            commandline::show_problem_arguments_message();
            exit(1)
        }
    }

    println!("**** Request summary ****");
    println!("Server address : {}", server_socket);
    println!("Requested role : {}", role);

    // *********** 2. get local system information
    let local_conf: LocalSystemConfig = localsystem::get_local_system_conf();
    println!("Local config :");
    println!("  - OS name : {}", local_conf.osname);
    println!("  - OS version : {}", local_conf.osversion);
    println!("  - Hostname : {}", local_conf.hostname);
    println!("  - HostID : {}", local_conf.hostid);

    // *********** 3. verify request consistency with the local system
    println!("Checking your system compatibility...");
    let request_consistency = localsystem::check_request_feasability(&role, &local_conf);
    if request_consistency {
        println!("Your system is compatible with your request.");
        // Proceed with the rest
    } else {
        println!("The requirements are not met for your request.");
        exit(1);
    }
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

#[derive(Debug)]
pub struct LocalSystemConfig {
    osname: String,
    osversion: String,
    hostname: String,
    hostid: String,
    disks_infos: Vec<u64>, // Only stores the free space of each disk (unit : gb)
}
