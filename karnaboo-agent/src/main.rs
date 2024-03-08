use std::env;
use std::net::SocketAddr;
use std::os::unix::process::ExitStatusExt;
use std::process::{exit, Output, ExitStatus};
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
    let role :  String; // "client | diss | reps"
    let server_socket: SocketAddr;
    let wait_mode: bool;

    match commandline::command_line_parsing(args) {
        Ok((tmp_role, tmp_server_socket, tmp_wait_mode)) => {
            role = tmp_role;
            server_socket = tmp_server_socket;
            wait_mode = tmp_wait_mode;
        }
        Err(()) => {
            commandline::show_problem_arguments_message();
            exit(1)
        }
    }

    if !wait_mode {

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

    }
        
    println!("**** Waiting for instructions from the Karnaboo server ****");
    let final_instructions: FinalInstructions = networking::get_instructions_from_server();


    println!("**** Executing the final instructions ****");
    let script_output = match Command::new("sh")
        .arg("-c")
        .arg(format!("{}", final_instructions.script_content))
        .output()
        {
            Ok(output) => {
                output
            }
            Err(e) => {
                println!("The script could not be executed : {:?}", e);
                Output {
                    status: ExitStatus::from_raw(99),
                    stdout: vec![],
                    stderr: format!("{:?}", e).into()
                }
            }
        };


    println!("**** Sending the execution results back to the Karnaboo server ****");
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