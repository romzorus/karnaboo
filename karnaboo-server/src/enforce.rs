use std::net::SocketAddr;
use std::io::{Read, Write};
use std::net::TcpStream;
use arangors::uclient::reqwest::ReqwestClient;
use arangors::Connection;
use arangors::Database;
use config::{self, Config, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::process::exit;
use std::net::TcpListener;

use crate::database::{NodeHostRequest, NodeReps, NodeDiss, NodeClient};
use crate::configuration::{DatabaseInfo, Networking};
use crate::database::NodeOs;

pub async fn enforce(db_info: &DatabaseInfo, networking_info: &Networking) {
    let db_connection = Connection::establish_basic_auth(
        format!(
            "http://{}:{}",
            &db_info.arangodb_server_address, &db_info.arangodb_server_port
        )
        .as_str(),
        &db_info.login,
        &db_info.password,
    )
    .await
    .unwrap();

    let db_connector = db_connection.db(&db_info.db_name).await.unwrap();

    // Starting with the REPS
    println!("Enforcing the REPS...");
    let reps_list: Vec<NodeReps> = db_connector
        .aql_str("FOR host IN reps RETURN host")
        .await
        .unwrap();

        for reps in reps_list.into_iter() {
        let exec_return = enforce_specific_host(&db_connector, reps._key.as_str(), "reps", format!("{}:9016", reps.ip).parse().unwrap(), networking_info).await;
        println!("      ** Exec return : START");
        println!("{}", exec_return.stdout);
        println!("      ** Exec return : END");
    }

    // Pursuing with the DISS
    println!("Enforcing the DISS...");
    let diss_list: Vec<NodeDiss> = db_connector
        .aql_str("FOR host IN diss RETURN host")
        .await
        .unwrap();

        for diss in diss_list.into_iter() {
        let exec_return = enforce_specific_host(&db_connector, diss._key.as_str(), "diss", format!("{}:9016", diss.ip).parse().unwrap(), networking_info).await;
        println!("      ** Exec return : START");
        println!("{}", exec_return.stdout);
        println!("      ** Exec return : END");
    }

    // Ending with the clients
    println!("Enforcing the clients...");
    let client_list: Vec<NodeClient> = db_connector
        .aql_str("FOR host IN clients RETURN host")
        .await
        .unwrap();

        for client in client_list.into_iter() {
        let exec_return = enforce_specific_host(&db_connector, client._key.as_str(), "client", format!("{}:9016", client.ip).parse().unwrap(), networking_info).await;
        println!("      ** Exec return : START");
        println!("{}", exec_return.stdout);
        println!("      ** Exec return : END");
    }

}

pub async fn enforce_specific_host(db_connector: &Database<ReqwestClient>, host_key: &str, role: &str, host_socket: SocketAddr, networking_info: &Networking) -> ExecutionResult {
    println!("  - Enforcing a {} at {}", role, host_socket.ip());

    // 1. Get the appropriate script
    let mut generic_instructions = FinalInstructions { script_content: String::from("") };
    match get_script_from_db(&db_connector, host_key, &role).await {
        Ok(content) => {
            generic_instructions = content;
        }
        Err(e) => {
            println!("Unable to get the appropriate script to enforce this role to this host");
            println!("{}", e);
            exit(1);
        }
    };

    // 2. Adapt the script to the specific topology of the host
    let final_instructions = adapt_instruction( role, generic_instructions);

    // 3. Send this script to the host
    send_script_to_host(host_socket, final_instructions);

    // 4. Wait for its return
    let host_exec_result = wait_for_host_exec_return(networking_info);

    // 5. Return that result
    host_exec_result
}



pub fn wait_for_host_exec_return(networking_info: &Networking) -> ExecutionResult {
    // Open socket
    let listener = TcpListener::bind(format!("{}:9016", networking_info.server_address)).expect(format!("Unable to open socket at {}:9016", networking_info.server_address).as_str());
    let (mut srv_stream, _srv_socket) = listener.accept().expect("Unable to establish connexion");

    // Get serialized data
    let mut buffer: [u8; 2048] = [0; 2048];
    let size = srv_stream
        .read(&mut buffer)
        .expect("Unable to read from TcpStream");
    let serialized_content = String::from_utf8_lossy(&buffer[..size]);

    // Deserialize
    let execution_result: ExecutionResult =
    serde_json::from_str(&serialized_content)
        .expect("Unable to deserialize data received from TcpStream"); 

    execution_result
}

pub fn adapt_instruction(role: &str, generic_instructions: FinalInstructions) -> FinalInstructions {
    // Do something
    println!("ADAPT_INSTRUCTION : TESTING PHASE");
    generic_instructions
}

pub async fn get_script_from_db(db_connector: &Database<ReqwestClient>, host_key: &str, role: &str) -> Result<FinalInstructions, String> {
    // Go from host to its connected OS then to the appropriate script connected to this OS

    // Which OS the host is connected to ?
    let host_os: Vec<String> = db_connector
        .aql_str(format!("FOR link IN uses_os FILTER link._from == \"{}/{}\" RETURN link._to", if role == "client" { "clients"} else { role }, host_key).as_str())
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
        Err(format!("OS connected to none or too much scripts. Script_result = {:?}", script_result))
    } else {
        Ok(FinalInstructions { script_content: script_result[0].clone()})
    }

    
}

// This function is used to fulfill the database.
// This function takes an os (md5 hash taken from repo-sources.yml)
// and a role and returns the appropriate script as a String
// to enforce this role to this os
pub fn get_script_from_source_file(role: &str, os: &str) -> Result<Script, String> {
    // Opening the script bank
    let config_builder = Config::builder()
        .add_source(File::new("../script_bank.yml", FileFormat::Yaml))
        .build()
        .unwrap();
    let script_bank = config_builder.try_deserialize::<ScriptBank>().unwrap();

    for script in script_bank.list.into_iter() {
        if (script.role == role) && script.compatible_with.contains(&os.to_string()) {
            return Ok(script);
        }
    }

    Err(String::from("No compatible script found !"))
}

// This function handles the networking part of sending the instructions to the host
pub fn send_script_to_host(host_socket: SocketAddr, final_instructions: FinalInstructions) {

     // Serialization before sending to socket
    let serialized_instructions = serde_json::to_string(&final_instructions).unwrap();

    let mut stream_client =
    // TcpStream::connect(host_socket).expect("Unable to connect to host\'s agent");
    TcpStream::connect(format!("{}:9017", host_socket.ip())).expect("Unable to connect to host\'s agent");

    stream_client
        .write(&serialized_instructions.as_bytes())
        .expect("Unable to send data to the host");
}


#[derive(Deserialize, Debug)]
pub struct ScriptBank {
    pub list: Vec<Script>,
}

#[derive(Deserialize, Debug)]
pub struct Script {
    pub _key: String,
    pub role: String,
    pub content: String,
    pub compatible_with: Vec<String>,
}

#[derive(Serialize)]
pub struct FinalInstructions {
    pub script_content: String
}

#[derive(Deserialize, Debug)]
pub struct ExecutionResult {
    pub exit_status: String,
    pub stdout: String,
    pub stderr: String
}