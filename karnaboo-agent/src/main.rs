use std::env;
use std::net::SocketAddr;
use std::process::exit;

mod commandline;
mod localsystem;
mod networking;

use crate::localsystem::LocalSystemConfig;
use crate::networking::{NodeHostRequest, NodeClient, NodeDiss, NodeReps};

fn main() {
    /* Global workflow :
        1. parse the request
        2. get local system information
        3. verify request consistency regarding system information
        4. send request
        5. wait for reception confirmation
        6. wait for decision and technical informations
        7. implement the decision with technical informations
    */
    // *********** 1. parse the request
    let args: Vec<String> = env::args().collect();
    let mut role = String::with_capacity(6); // "client | diss | reps"
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
    if !request_consistency {
        println!("The requirements are not met for your request.");
        exit(1);
    }
    println!("Your request is compatible with your system.");

    // *********** 4. send request
    // Building content
    let content: NodeHostRequest;
    if role == "client" {
        content = NodeHostRequest::Client(NodeClient {
            hostname: local_conf.hostname,
            ip: String::from("0.0.0.0"),
            osname: local_conf.osname,
            osversion: local_conf.osversion,
            hostid: local_conf.hostid
        });
    } else if role == "diss" {
        content = NodeHostRequest::Diss(NodeDiss {
            hostname: local_conf.hostname,
            ip: String::from("0.0.0.0"),
            osname: local_conf.osname,
            osversion: local_conf.osversion,
            hostid: local_conf.hostid
        });
    } else {
        content = NodeHostRequest::Reps(NodeReps {
            hostname: local_conf.hostname,
            ip: String::from("0.0.0.0"),
            osname: local_conf.osname,
            osversion: local_conf.osversion,
            hostid: local_conf.hostid
        });
    }

    networking::send_request(content, server_socket);

    // *********** 5. wait for reception confirmation
    // *********** 6. wait for decision and technical informations
    // *********** 7. implement the decision with technical informations
}
