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


    println!("**** Getting local host informations ****");
    let local_conf: LocalSystemConfig = localsystem::get_local_system_conf();


    println!("**** Checking your system compatibility ****");
    localsystem::check_request_feasability(&role, &local_conf);


    println!("**** Sending registration request ****");
    networking::send_request(server_socket, role, local_conf);


    // *********** 5. wait for reception confirmation
    // TBD eventually : for the time being, we assume every request is granted


    println!("**** Waiting for instructions from the Karnaboo server ****");
    let final_instructions: FinalInstructions = networking::get_instructions_from_server();


    // *********** 7. implement the decision with received technical informations
    println!("**** Executing the final instructions ****");
    let script_output = Command::new("sh")
        .arg("-c")
        .arg(format!("{}", final_instructions.script_content))
        .output()
        .expect("Wrong command");

        
    println!("**** Execute the final instructions ****");
    networking::send_exec_result_to_server(script_output, server_socket);

}

#[derive(Deserialize, Debug)]
pub struct FinalInstructions {
    pub script_content: String
}

#[derive(Serialize)]
pub struct ExecutionResult {
    pub exit_status: String,
    pub stdout: String,
    pub stderr: String
}