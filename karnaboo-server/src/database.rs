/*
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use arangors::uclient::reqwest::ReqwestClient;
use arangors::Database;
use arangors::{ClientError, Connection};
use colored::Colorize;
use futures::lock::Mutex;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, value::Value};
use std::io::{stdout, Write};
use std::sync::Arc;

use crate::commands::yes_or_no_question;
use crate::configuration::DatabaseInfo;

pub async fn direct_user_interaction_with_db(db_info: &DatabaseInfo) -> Result<()> {
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

    let db = db_connection.db(&db_info.db_name).await.unwrap();

    println!("{}", "Connection to database established".bold().yellow());

    let mut rl = DefaultEditor::new()?;

    let mut user_input = String::with_capacity(150);

    loop {
        let readline = rl.readline(format!("{}", "(db mode) $ ".bold().yellow()).as_str());

        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                user_input = line.clone();
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl-C
                break;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl-D
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }

        // Command analysis
        let user_input_str = user_input.as_str();

        if user_input_str.is_empty() {
            continue;
        } else if ["exit", "ex", "quit", "q"].contains(&user_input_str) {
            break;
        } else {
            // Send command to database

            let _result_command: Vec<serde_json::Value> = match db.aql_str(user_input_str).await {
                Ok(content) => {
                    println!("Database return");
                    println!("{:?}", content);
                    content
                }
                Err(e) => {
                    println!("{}", "Invalid AQL query".red());
                    println!("{:?}", e);
                    vec![json!([""])]
                }
            };
        }
        user_input.clear();
    }

    println!("Back to main command mode");
    Ok(())
}

pub async fn db_create_client(db_info: &DatabaseInfo, host_info: NodeClient) -> Result<()> {
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

    let db = db_connection.db(&db_info.db_name).await.unwrap();

    let client_creation_query = format!(
        "insert {{\"hostname\": \"{}\", \"ip\": \"{}\" }} into clients",
        host_info.hostname, host_info.ip
    );

    let _result_command: Vec<serde_json::Value> =
        match db.aql_str(client_creation_query.as_str()).await {
            Ok(content) => {
                println!("Database return");
                println!("{:?}", content);
                content
            }
            Err(e) => {
                println!("{}", "Invalid AQL query".red());
                println!("{:?}", e);
                vec![json!([""])]
            }
        };
    Ok(())
}

pub async fn db_create_diss(db_info: &DatabaseInfo, host_info: NodeClient) -> Result<()> {
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

    let db = db_connection.db(&db_info.db_name).await.unwrap();

    let diss_creation_query = format!(
        "insert {{\"hostname\": \"{}\", \"ip\": \"{}\" }} into diss",
        host_info.hostname, host_info.ip
    );

    let _result_command: Vec<serde_json::Value> =
        match db.aql_str(diss_creation_query.as_str()).await {
            Ok(content) => {
                println!("Database return");
                println!("{:?}", content);
                content
            }
            Err(e) => {
                println!("{}", "Invalid AQL query".red());
                println!("{:?}", e);
                vec![json!([""])]
            }
        };
    Ok(())
}

pub async fn db_create_reps(db_info: &DatabaseInfo, host_info: NodeClient) -> Result<()> {
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

    let db = db_connection.db(&db_info.db_name).await.unwrap();

    let reps_creation_query = format!(
        "insert {{\"hostname\": \"{}\", \"ip\": \"{}\" }} into diss",
        host_info.hostname, host_info.ip
    );

    let _result_command: Vec<serde_json::Value> =
        match db.aql_str(reps_creation_query.as_str()).await {
            Ok(content) => {
                println!("Database return");
                println!("{:?}", content);
                content
            }
            Err(e) => {
                println!("{}", "Invalid AQL query".red());
                println!("{:?}", e);
                vec![json!([""])]
            }
        };
    Ok(())
}

pub async fn answer_requests(
    waiting_requests: &Arc<Mutex<Vec<NodeHostRequest>>>,
    db_info: &DatabaseInfo,
) {
    let mut waiting_requests_contents = waiting_requests.lock().await;
    let total_number = &waiting_requests_contents.len();

    if *total_number == 0 as usize {
        println!("{}", "No request waiting.".bold().blue());
        return;
    }

    let list_of_waiting_requests = waiting_requests_contents.clone();

    for (i, req) in list_of_waiting_requests.into_iter().enumerate() {
        print!(
            "{}",
            format!("Request {} / {} : ", i + 1, total_number)
                .bold()
                .blue()
        );

        match req {
            NodeHostRequest::Client(host_info) => {
                println!(
                    "{}",
                    format!(
                        "\'{}\' at {} => New client ? ",
                        host_info.hostname, host_info.ip
                    )
                    .bold()
                    .blue()
                );

                if yes_or_no_question() {
                    // Send data to database and erase the request
                    let return_db_client_creation = db_create_client(&db_info, host_info);
                    let _ = return_db_client_creation.await;

                    println!("{}", "Client added to database".bold().blue());
                    waiting_requests_contents.remove(0);
                } else {
                    // Simply erasing the request
                    waiting_requests_contents.remove(0);
                    println!("{}", "Request dropped".bold().blue());
                }
            }
            NodeHostRequest::Diss(host_info) => {
                println!(
                    "{}",
                    format!(
                        "\'{}\' at {} => New DISS ? ",
                        host_info.hostname, host_info.ip
                    )
                    .bold()
                    .blue()
                );

                if yes_or_no_question() {
                    // Send data to database and erase the request

                    println!("{}", "DISS added to database".bold().blue());
                    waiting_requests_contents.remove(0);
                } else {
                    // Simply erasing the request
                    waiting_requests_contents.remove(0);
                }
            }
            NodeHostRequest::Reps(host_info) => {
                println!(
                    "{}",
                    format!(
                        "\'{}\' at {} => New REPS ? ",
                        host_info.hostname, host_info.ip
                    )
                    .bold()
                    .blue()
                );

                if yes_or_no_question() {
                    // Send data to database and erase the request

                    println!("{}", "REPS added to database".bold().blue());
                    waiting_requests_contents.remove(0);
                } else {
                    // Simply erasing the request
                    waiting_requests_contents.remove(0);
                }
            }
        }
    }
}

pub async fn db_check(db_info: &DatabaseInfo) -> Result<()> {
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

    let db = db_connection.db(&db_info.db_name).await.unwrap();

    /*
        Things to check in the database :
            - the collections must exist (nodes and edges) and with the proper names
            - each client must be connected to one (and only one) DISS and one (and only one) an OS
            - each DISS must be connected to one (and only one) REPS and a least one OS
            - each REPS must be connected to a least one OS
            - for each OS connected to a client, it has to be connected to at least one DISS and one REPS
            - a DISS must be compatible with the OS of the client = triangle (edges) between client-DISS-OS
            - a REPS must be compatible with the OS of its DISS = triangle (edges) between REPS-DISS-OS
            - for each client, there must be a path client->DISS->REPS, otherwise, the client won't get updates
    */

    /* If one of the items is wrong, prompt the administrator and ask to remediate the situation (propose a solution when possible) */

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeClient {
    pub hostname: String,
    pub ip: String,
    pub osname: String,
    pub osversion: String,
    pub hostid: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeReps {
    pub hostname: String,
    pub ip: String,
    pub osname: String,
    pub osversion: String,
    pub hostid: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeDiss {
    pub hostname: String,
    pub ip: String,
    pub osname: String,
    pub osversion: String,
    pub hostid: String
}

pub struct NodeOs {
    pub name: String,
    pub version: String,
}

pub struct EgdeDissCompatibleWith {
    pub diss: NodeDiss,
    pub os: NodeOs,
}
pub struct EdgeHandles {
    pub diss: NodeDiss,
    pub client: NodeClient,
}
pub struct EdgeRedistributesTo {
    pub reps: NodeReps,
    pub diss: NodeDiss,
}
pub struct EdgeRepsCompatibleWith {
    pub reps: NodeReps,
    pub os: NodeOs,
}
pub struct EdgeUsesOs {
    pub client: NodeClient,
    pub os: NodeOs,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NodeHostRequest {
    Client(NodeClient),
    Diss(NodeDiss),
    Reps(NodeReps),
}

// pub async fn thread_database_interact() {
// !!! Retirer le mot de passe codé en dur du code source via une lecture de fichier de conf externe
// let db_connection =
//     Connection::establish_basic_auth("http://127.0.0.1:8529", "root", "arangodb")
//         .await
//         .unwrap();

// let db = db_connection.db("_system").await.unwrap();

// let collection_clients = db.collection("clients").await.unwrap();
// let collection_reps = db.collection("reps").await.unwrap();
// let collection_diss = db.collection("diss").await.unwrap();
// let collection_os = db.collection("os").await.unwrap();

// }

/* Notes for upcoming functions

Useful AQL queries :

- Which clients are handled by [specific DISS] ?
FOR val IN handles FILTER val._from == "diss/3084" RETURN val._to

- Which DISS are compatible with [specific OS] ?
FOR val IN diss_compatible_with FILTER val._to == "os/3291" RETURN val._from

- Which clients are not connected to a DISS (to any of the existing DISSs) ?
FOR client IN clients FILTER client._key NOT IN (FOR handle IN handles RETURN LTRIM(handle._to, "clients/")) RETURN client.hostname

UPSERT: Update/replace an existing document, or create it in the case it does not exist.
*/
