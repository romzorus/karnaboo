use serde::{Deserialize, Serialize};
use colored::Colorize;
use std::{env, process::exit};

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
    let mut server = String::with_capacity(21); // "IP:port"

    if args.len() == 1 {
        show_problem_arguments_message();
        exit(1);
    } else {
        let args_cloned = args.clone();
        for (i, _arg) in args_cloned.into_iter().enumerate() {

            // Looking for server and role arguments
            if ["-s", "--server"].contains(&args[i].as_str()) { // There needs to be a following argument...
                if args.len() >= (i+2) { // (length=i+1) and we are looking for the next argument so (i+1+1) = (i+2)
                    server = args[i+1].clone();
                } else { // ... otherwise :
                    show_problem_arguments_message();
                    exit(1);
                }
            } else if ["-r", "--role"].contains(&args[i].as_str()) { // There needs to be a following argument...
                if args.len() >= (i+2) { // (length=i+1) and we are looking for the next argument so (i+1+1) = (i+2)
                    role = args[i+1].clone();
                    if !["client", "CLIENT", "Client", "DISS", "diss", "REPS", "reps"].contains(&role.as_str()) { // if argument not consistent with possible roles, just abort
                        show_problem_arguments_message();
                        exit(1);
                    }
                } else { // ... otherwise :
                    show_problem_arguments_message();
                    exit(1);
                }
            } else if ["-h", "--help"].contains(&args[i].as_str()) {
                show_help_message();
                break;
            }
        }

        // If server or role argument is missing, abort
        if server.is_empty() || role.is_empty() {
            show_problem_arguments_message();
            exit(1);
        }

    }
    
    println!("Server : {}", server);
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

fn show_help_message() {

    println!("Please use the karnaboo agent as follows :");
    println!(r"██████████████████████████████████████████████████████");
    println!(r"████ {} ████", "karnaboo-agent -s [IP_server:port] -r [role]");
    println!(r"██████████████████████████████████████████████████████");
    println!("  -s --server : server address and port");
    println!("  -r --role : role requested for this machine");
    println!("");
    println!("Example :");
    println!("We want this machine to become a DISS. The karnaboo server");
    println!("is reachable at 192.168.1.1:9015.");
    println!("");
    println!("$ {}", "karnaboo-agent -s 192.168.1.1:9015 -r diss");
    println!("");
    println!("Possibles roles are client, diss or reps.");
    println!("");
}

fn show_problem_arguments_message() {
    println!("{}", "Missing or wrong arguments".bold().red());
    show_help_message();
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

/* ***** Note for later *****

Example for server argument parsing :
use std::net::SocketAddr;

fn main() {
    let server_details = "127.0.0.1:80";
    let server: SocketAddr = server_details
        .parse()
        .expect("Unable to parse socket address");
    println!("{:?}", server);
} */