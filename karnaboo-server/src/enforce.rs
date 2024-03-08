use arangors::GenericConnection;
/*
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/
use arangors::uclient::reqwest::ReqwestClient;
use arangors::Connection;
use arangors::Database;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;

use crate::configuration::{DatabaseInfo, Networking};
use crate::database::{NodeClient, NodeDiss, NodeReps};

pub async fn enforce(db_info: &DatabaseInfo, networking_info: &Networking) {
    let db_connection: GenericConnection<ReqwestClient>;
    match Connection::establish_basic_auth(
        format!(
            "http://{}:{}",
            &db_info.arangodb_server_address, &db_info.arangodb_server_port
        )
        .as_str(),
        &db_info.login,
        &db_info.password,
    )
    .await
    {
        Ok(connection_tmp) => {
            db_connection = connection_tmp;
        }
        Err(err) => {
            println!(
                "{}",
                format!("[Enforce] unable to connect to database server : {:?}", err).red()
            );
            return;
        }
    }

    let db_connector: Database<ReqwestClient>;
    match db_connection.db(&db_info.db_name).await {
        Ok(connector_tmp) => {
            db_connector = connector_tmp;
        }
        Err(err) => {
            println!(
                "{}",
                format!("[Enforce] unable to connect to database : {:?}", err).red()
            );
            return;
        }
    }

    // Beginning with the REPS
    println!("Enforcing the REPS...");
    match db_connector
        .aql_str::<NodeReps>("FOR host IN reps RETURN host")
        .await
    {
        Ok(reps_list) => {
            for host in reps_list.into_iter() {
                let host_ip: SocketAddr;
                match format!("{}:9016", host.ip).parse() {
                    Ok(data_tmp) => {
                        host_ip = data_tmp;
                    }
                    Err(_) => {
                        println!(
                            "{}",
                            format!(
                                "[Enforce] unable to parse IP of {}. Skipping",
                                host.hostname
                            )
                            .red()
                        );
                        continue;
                    }
                }

                println!("      ** Exec return : START");
                match enforce_specific_host(
                    &db_connector,
                    host._key.as_str(),
                    "reps",
                    host_ip,
                    networking_info,
                )
                .await
                {
                    Ok(exec_result_tmp) => {
                        println!("{}", exec_result_tmp.stdout);
                    }
                    Err(err) => {
                        println!("Enforcing failed : {:?}", err);
                    }
                }
                println!("      ** Exec return : END");
            }
        }
        Err(_) => {
            println!("{}", "[Enforce] unable to get REPS list. Skipping".red());
        }
    }

    // Pursuing with the DISS
    println!("Enforcing the DISS...");
    match db_connector
        .aql_str::<NodeDiss>("FOR host IN diss RETURN host")
        .await
    {
        Ok(diss_list) => {
            for host in diss_list.into_iter() {
                let host_ip: SocketAddr;
                match format!("{}:9016", host.ip).parse() {
                    Ok(data_tmp) => {
                        host_ip = data_tmp;
                    }
                    Err(_) => {
                        println!(
                            "{}",
                            format!(
                                "[Enforce] unable to parse IP of {}. Skipping",
                                host.hostname
                            )
                            .red()
                        );
                        continue;
                    }
                }

                println!("      ** Exec return : START");
                match enforce_specific_host(
                    &db_connector,
                    host._key.as_str(),
                    "diss",
                    host_ip,
                    networking_info,
                )
                .await
                {
                    Ok(exec_result_tmp) => {
                        println!("{}", exec_result_tmp.stdout);
                    }
                    Err(err) => {
                        println!("Enforcing failed : {:?}", err);
                    }
                }
                println!("      ** Exec return : END");
            }
        }
        Err(_) => {
            println!("{}", "[Enforce] unable to get DISS list. Skipping".red());
        }
    }

    // Concluding with the clients
    println!("Enforcing the clients...");
    match db_connector
        .aql_str::<NodeClient>("FOR host IN clients RETURN host")
        .await
    {
        Ok(client_list) => {
            for host in client_list.into_iter() {
                let host_ip: SocketAddr;
                match format!("{}:9016", host.ip).parse() {
                    Ok(data_tmp) => {
                        host_ip = data_tmp;
                    }
                    Err(_) => {
                        println!(
                            "{}",
                            format!(
                                "[Enforce] unable to parse IP of {}. Skipping",
                                host.hostname
                            )
                            .red()
                        );
                        continue;
                    }
                }

                println!("      ** Exec return : START");
                match enforce_specific_host(
                    &db_connector,
                    host._key.as_str(),
                    "client",
                    host_ip,
                    networking_info,
                )
                .await
                {
                    Ok(exec_result_tmp) => {
                        println!("{}", exec_result_tmp.stdout);
                    }
                    Err(err) => {
                        println!("Enforcing failed : {:?}", err);
                    }
                }
                println!("      ** Exec return : END");
            }
        }
        Err(_) => {
            println!("{}", "[Enforce] unable to get CLIENT list. Skipping".red());
        }
    }
}

pub async fn enforce_specific_host(
    db_connector: &Database<ReqwestClient>,
    host_key: &str,
    role: &str,
    host_socket: SocketAddr,
    networking_info: &Networking,
) -> Result<ExecutionResult, ErrorKinds> {
    println!("  - Enforcing a {} at {}", role, host_socket.ip());

    // 1. Get the appropriate script
    match get_script_from_db(&db_connector, host_key, &role).await {
        Ok(generic_instructions) => {
            // 2. Adapt the script to the specific topology of the host
            match adapt_instruction(&db_connector, role, host_key, generic_instructions).await {
                Ok(final_instructions) => {
                    // 3. Send this script to the host
                    match send_script_to_host(host_socket, final_instructions) {
                        Ok(_) => {
                            // 4. Wait for its return
                            let host_exec_result = wait_for_host_exec_return(networking_info);
                            
                            if host_exec_result.exit_status == ExitStatus::from_raw(99).to_string() {
                                Err(ErrorKinds::FailedExecution)
                            } else {
                                 // 5. Return that result
                                Ok(host_exec_result)
                            }
                        }
                        Err(kind) => match kind {
                            ErrorKinds::FailedSerialization => {
                                println!(
                                    "[Enforce] unable to serialize data to send to remote host"
                                );
                                Err(ErrorKinds::FailedSerialization)
                            }
                            ErrorKinds::FailedSocketConnection => {
                                println!("[Enforce] unable to connect to remote host");
                                Err(ErrorKinds::FailedSocketConnection)
                            }
                            ErrorKinds::FailedSendingData => {
                                println!("[Enforce] unable to write data to TcpStream");
                                Err(ErrorKinds::FailedSendingData)
                            }
                            _ => {
                                println!("[Enforce] problem encountered");
                                Err(ErrorKinds::GenericError)
                            }
                        },
                    }
                }
                Err(_) => {
                    println!("[Enforce] unable to produce a specific script for this host");
                    Err(ErrorKinds::FailedSpecificScriptBuilding)
                }
            }
        }
        Err(err) => {
            println!(
                "[Enforce] unable to get the script for this (host,role) : {}",
                err
            );
            Err(ErrorKinds::FailedScriptRecovery)
        }
    }
}

pub fn wait_for_host_exec_return(networking_info: &Networking) -> ExecutionResult {
    // Open socket
    let listener = TcpListener::bind(format!("{}:9016", networking_info.server_address)).expect(
        format!(
            "Unable to open socket at {}:9016",
            networking_info.server_address
        )
        .as_str(),
    );
    let (mut srv_stream, _srv_socket) = listener.accept().expect("Unable to establish connexion");

    // Get serialized data
    let mut buffer: [u8; 2048] = [0; 2048];
    let size = srv_stream
        .read(&mut buffer)
        .expect("Unable to read from TcpStream");
    let serialized_content = String::from_utf8_lossy(&buffer[..size]);

    // Deserialize
    match serde_json::from_str(&serialized_content) {
        Ok(content) => content,
        Err(e) => ExecutionResult {
            exit_status: "Error".to_string(),
            stdout: "Unable to deserialize data received from TcpStream".to_string(),
            stderr: e.to_string(),
        },
    }
}

pub async fn adapt_instruction(
    db_connector: &Database<ReqwestClient>,
    role: &str,
    host_key: &str,
    generic_instructions: FinalInstructions,
) -> Result<FinalInstructions, String> {
    match role {
        "client" => {
            // A client needs the IP address of its DISS
            let ip_diss: Vec<String> = db_connector
                .aql_str(format!("FOR link IN handles FILTER link._to == \"clients/{}\" RETURN document(link._from).ip",
                host_key).as_str())
                .await
                .unwrap();
            if ip_diss.len() != 1 {
                println!("This client is connected to none or too much DISS. Abort");
                Err(format!("ip_diss = {:?}", ip_diss))
            } else {
                Ok(FinalInstructions {
                    script_content: generic_instructions
                        .script_content
                        .replace("$IP_DISS", ip_diss[0].as_str()),
                })
            }
        }
        "diss" => {
            // A DISS needs the IP address of its REPS and [..?]
            let ip_reps: Vec<String> = db_connector
            .aql_str(format!("FOR link IN redistributes_to FILTER link._to == \"diss/{}\" RETURN document(link._from).ip",
            host_key).as_str())
            .await
            .unwrap();
            if ip_reps.len() != 1 {
                println!("This DISS is connected to none or too much REPS. Abort");
                Err(format!("ip_reps = {:?}", ip_reps))
            } else {
                Ok(FinalInstructions {
                    script_content: generic_instructions
                        .script_content
                        .replace("$IP_REPS", ip_reps[0].as_str()),
                })
            }
        }
        "reps" => {
            // A REPS needs [.. ?]
            Ok(generic_instructions)
        }
        _ => Err(String::from("Error reading the role argument")),
    }
}

pub async fn get_script_from_db(
    db_connector: &Database<ReqwestClient>,
    host_key: &str,
    role: &str,
) -> Result<FinalInstructions, String> {
    // Go from host to its connected OS then to the appropriate script connected to this OS

    // Which OS the host is connected to ?
    let host_os: Vec<String> = db_connector
        .aql_str(
            format!(
                "FOR link IN uses_os FILTER link._from == \"{}/{}\" RETURN link._to",
                if role == "client" { "clients" } else { role },
                host_key
            )
            .as_str(),
        )
        .await
        .unwrap();

    let mut script_result: Vec<String> = vec![];

    if host_os.len() != 1 {
        println!("This host is connected to none or too much OS. Abort");
        println!("Host_os = {:?}", host_os);
    } else {
        script_result = db_connector
        .aql_str(format!("FOR link IN script_compatible_with FILTER link._to == \"{}\" FILTER document(link._from).role == \"{}\" RETURN document(link._from).content",
            host_os[0], role).as_str())
        .await
        .unwrap();
    }

    if script_result.len() != 1 {
        Err(format!(
            "OS connected to none or too much scripts. Script_result = {:?}",
            script_result
        ))
    } else {
        Ok(FinalInstructions {
            script_content: script_result[0].clone(),
        })
    }
}

// This function handles the networking part of sending the instructions to the host
// pub fn send_script_to_host_old(
//     host_socket: SocketAddr,
//     final_instructions: FinalInstructions,
// ) {
// Serialization before sending to socket
// let serialized_instructions = serde_json::to_string(&final_instructions).unwrap();

// let mut stream_client = TcpStream::connect(format!("{}:9017", host_socket.ip())).unwrap();

// stream_client
//     .write(&serialized_instructions.as_bytes())
//     .expect("Unable to send data to the host");

// }

// This function handles the networking part of sending the instructions to the host
pub fn send_script_to_host(
    host_socket: SocketAddr,
    final_instructions: FinalInstructions,
) -> Result<usize, ErrorKinds> {
    // Serialization before sending to socket
    match serde_json::to_string(&final_instructions) {
        Ok(serialized_instructions) => {
            match TcpStream::connect(format!("{}:9017", host_socket.ip())) {
                Ok(mut stream_client) => {
                    match stream_client.write(&serialized_instructions.as_bytes()) {
                        Ok(usize) => Ok(usize),
                        Err(_) => Err(ErrorKinds::FailedSendingData),
                    }
                }
                Err(_) => Err(ErrorKinds::FailedSocketConnection),
            }
        }
        Err(_) => Err(ErrorKinds::FailedSerialization),
    }
}

#[derive(Serialize)]
pub struct FinalInstructions {
    pub script_content: String,
}

#[derive(Deserialize, Debug)]
pub struct ExecutionResult {
    pub exit_status: String,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug)]
pub enum ErrorKinds {
    FailedSerialization,
    FailedSocketConnection,
    FailedSendingData,
    FailedSpecificScriptBuilding,
    FailedScriptRecovery,
    FailedExecution,
    GenericError,
}
