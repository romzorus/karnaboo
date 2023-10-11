use std::env;
use std::net::SocketAddr;
use std::process::{exit, Output};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use std::process::Command;


mod commandline;
mod localsystem;
mod networking;

use crate::localsystem::LocalSystemConfig;
use crate::networking::{NodeHostRequest, NodeClient, NodeDiss, NodeReps};

/* Global workflow :
    1. parse the request
    2. get local system information
    3. verify request consistency regarding system information
    4. send request
    5. wait for reception confirmation
    6. wait for decision and technical informations
    7. implement the decision with technical informations
*/

fn main() {

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
    println!("  - Key : {}", local_conf._key);

    // *********** 3. verify request consistency with the local system
    println!("**** Checking your system compatibility... ****");
    let request_consistency = localsystem::check_request_feasability(&role, &local_conf);
    if !request_consistency {
        println!("The requirements are not met for your request.");
        exit(1);
    }
    println!("Your system is compatible with your request.");

    // *********** 4. send request
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

    networking::send_request(content, server_socket);

    // *********** 5. wait for reception confirmation
    // TBD eventually : for the time being, we assume every request is granted

    // *********** 6. wait for decision and technical informations
    let listener = TcpListener::bind("127.0.0.1:9016").expect("Unable to open socket 127.0.0.1:9016");
    let (mut srv_stream, _srv_socket) = listener.accept().expect("Unable to establish connexion");

    let mut buffer: [u8; 2048] = [0; 2048];
    let size = srv_stream
        .read(&mut buffer)
        .expect("Unable to read from TcpStream");
    let serialized_content = String::from_utf8_lossy(&buffer[..size]);
    let final_instructions: FinalInstructions =
        serde_json::from_str(&serialized_content)
            .expect("Unable to deserialize data received from TcpStream");


    // *********** 7. implement the decision with received technical informations
    println!("**** Execute the final instructions ****");
    let script_output = Command::new("sh")
        .arg("-c")
        .arg(final_instructions.script_content)
        .output()
        .expect("Wrong command");

    
    // *********** 8. send the result back to the server

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
        TcpStream::connect(server_socket).expect("Unable to connect to Karnaboo server");

    // Send to server the serialized request
    socket_srv
        .write(&serialized_script_output.as_bytes())
        .expect("Unable to send data through socket");

}

#[derive(Deserialize)]
struct FinalInstructions {
    script_content: String
}

#[derive(Serialize)]
struct ExecutionResult {
    exit_status: String,
    stdout: String,
    stderr: String
}