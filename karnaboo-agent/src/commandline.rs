use std::net::SocketAddr;
use std::process::exit;
use colored::Colorize;


pub fn command_line_parsing(args: Vec<String>) -> (String, SocketAddr) {

    let mut role = String::with_capacity(6); // "client | reps | diss"
    let mut server = String::with_capacity(21); // "IP:port"
    let mut server_socket: SocketAddr = "0.0.0.0:0".parse().unwrap();
    
    if args.len() > 1 {
        // It would be easier to just state "args.len() == 5" meaning server address + role
        // but the need for more arguments will probably emerge so it is written from the beginning
        // to easily add more options later.
        let args_cloned = args.clone();
        for (i, _arg) in args_cloned.into_iter().enumerate() {

            // Looking for server and role arguments
            if ["-s", "--server"].contains(&args[i].as_str()) { // There needs to be a following argument...
                if args.len() >= (i+2) { // (length=i+1) and we are looking for the next argument so (i+1+1) = (i+2)
                    server = args[i+1].clone();
            
                    match server.parse() {
                        Ok(socket) => {
                            server_socket = socket;
                        }
                        Err(e) => {
                            println!("{}", format!("{:?}", e).red().bold());
                            show_problem_arguments_message();
                            exit(1);
                        }
                    }

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

        // After parsing all the input, if either server or role argument is missing, abort
        if server.is_empty() || role.is_empty() {
            show_problem_arguments_message();
            exit(1);
        }

        (role, server_socket)

    } else {
        show_problem_arguments_message();
        exit(1);
    }
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