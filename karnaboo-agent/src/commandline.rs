use colored::Colorize;
use std::net::SocketAddr;

pub fn command_line_parsing(args: Vec<String>) -> Result<(String, SocketAddr, bool), ()> {
    let mut role = String::with_capacity(6); // "client | reps | diss"
    let mut server = String::with_capacity(21); // "IP:port"
    let mut server_socket: SocketAddr = "0.0.0.0:0".parse().unwrap();
    let mut wait_mode = false; // By default, we send a request first

    if args.len() > 1 {

        let args_cloned = args.clone();
        for (i, _arg) in args_cloned.into_iter().enumerate() {
            // Looking for server and role arguments
            if ["-s", "--server"].contains(&args[i].as_str()) {
                // There needs to be a following argument...
                if args.len() >= (i + 2) {
                    // (length=i+1) and we are looking for the next argument so (i+1+1) = (i+2)
                    server = args[i + 1].clone();

                    // If no port is specified, assume 9015
                    if !server.contains(":") {
                        server.push_str(":9015");
                    }

                    match server.parse() {
                        Ok(socket) => {
                            server_socket = socket;
                        }
                        Err(e) => {
                            println!("{}", format!("{:?}", e).red().bold());
                            return Err(());
                        }
                    }
                } else {
                    // ... otherwise :
                    return Err(());
                }
            } else if ["-r", "--role"].contains(&args[i].as_str()) {
                // There needs to be a following argument...
                if args.len() >= (i + 2) {
                    // (length=i+1) and we are looking for the next argument so (i+1+1) = (i+2)
                    role = args[i + 1].clone().to_lowercase();
                    if !["client", "diss", "reps"].contains(&role.as_str()) {
                        // if argument not consistent with possible roles, just abort
                        return Err(());
                    }
                } else {
                    return Err(());
                }
            } else if ["-h", "--help"].contains(&args[i].as_str()) {
                show_help_message();
                break;
            } else if ["-w", "--wait"].contains(&args[i].as_str()) {
                wait_mode = true;
            }
        }

        // After parsing all the input, if either server or role argument is missing, abort
        // The wait_mode is optionnal.
        if server.is_empty() || role.is_empty() {
            return Err(());
        } else {
            Ok((role, server_socket, wait_mode))
        }

    } else { // Missing arguments
        return Err(());
    }
}

fn show_help_message() {
    println!("Please use the karnaboo agent as follows :");
    println!(r"███████████████████████████████████████████████████████████");
    println!(r"████ karnaboo-agent [-w] -s [IP_server:port] -r [role] ████");
    println!(r"███████████████████████████████████████████████████████████");
    println!("  -s --server : server address and port");
    println!("  -r --role : role requested for this machine");
    println!("  -w --wait : directly wait for instructions without sending request first");
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

pub fn show_problem_arguments_message() {
    println!("{}", "Missing or wrong arguments".bold().red());
    show_help_message();
}
