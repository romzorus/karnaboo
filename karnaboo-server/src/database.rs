/*
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use arangors::{Connection, graph::{Graph, EdgeDefinition}};
use colored::Colorize;
use config::{self, Config, File, FileFormat};
use futures::lock::Mutex;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{sync::Arc, thread, time::Duration};
use std::process::Command;

use crate::commands::yes_or_no_question;
use crate::configuration::DatabaseInfo;
use crate::enforce::get_script_from_source_file;

// Let the user directly interact with the database via AQL queries ("AQL mode").
// With AQL queries, the user is limited and cannot create or delete database or collections.
pub async fn aql_mode(db_info: &DatabaseInfo) -> Result<()> {
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

    println!(
        "{}",
        "Connection to database established (AQL mode)"
            .bold()
            .yellow()
    );

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

        let req_clone_for_os = req.clone();

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

                if yes_or_no_question(true) {
                    // Send data to database and erase the request
                    // Create/update the client
                    let return_db_client_creation = db_create_update_client(&db_info, &host_info);
                    let _ = return_db_client_creation.await;
                    // Create/update the OS
                    let return_os_client_creation = db_create_update_os(db_info, &req_clone_for_os);
                    let _ = return_os_client_creation.await;
                    // Link the client to the OS

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

                if yes_or_no_question(true) {
                    // Send data to database and erase the request
                    let return_db_diss_creation = db_create_update_diss(&db_info, host_info);
                    let _ = return_db_diss_creation.await;
                    // Create/update the OS
                    let return_os_client_creation = db_create_update_os(db_info, &req_clone_for_os);
                    let _ = return_os_client_creation.await;
                    // Link the client to the OS

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

                if yes_or_no_question(true) {
                    // Send data to database and erase the request
                    let return_db_reps_creation = db_create_update_reps(&db_info, host_info);
                    let _ = return_db_reps_creation.await;
                    // Create/update the OS
                    let return_os_client_creation = db_create_update_os(db_info, &req_clone_for_os);
                    let _ = return_os_client_creation.await;
                    // Link the client to the OS

                    waiting_requests_contents.remove(0);
                } else {
                    // Simply erasing the request
                    waiting_requests_contents.remove(0);
                }
            }
        }
    }
}

pub async fn db_build(db_info: &DatabaseInfo) -> Result<()> {
    println!("");
    println!("--- Creating database ---");
    let _ = db_create_update_database(db_info).await;

    println!("");
    println!("--- Creating collections ---");

    for node_collection_name in ["clients", "diss", "reps", "os", "scripts"] {
        let _ = db_create_update_node_collection(db_info, node_collection_name).await;
    }

    for edge_collection_name in [
        "diss_compatible_with",
        "handles",
        "redistributes_to",
        "reps_compatible_with",
        "script_compatible_with",
        "uses_os",
    ] {
        let _ = db_create_update_edge_collection(db_info, edge_collection_name).await;
    }
    println!("");
    println!("--- Creating graph ---");

    let _ = db_create_graph(db_info).await;

    Ok(())
}

pub async fn db_check(db_info: &DatabaseInfo) -> Result<()> {

    println!("Database and collections checking...");

    // 1. Check database and collections existence and create them if necessary
    let _ = db_build(db_info).await;
        /*
        Required collections :
            Nodes :
            - clients
            - diss
            - reps
            - os
            - scripts

            Edges :
            - diss_compatible_with
            - handles
            - redistributes_to
            - reps_compatible_with
            - script_compatible_with
            - uses_os


        Things to check in the database :
            [X] the collections must exist (nodes and edges) and with the proper names
            [ ] each client must be connected to one (and only one) DISS and one (and only one) an OS
            [ ] each DISS must be connected to one (and only one) REPS and a least one OS
            [ ] each REPS must be connected to a least one OS
            [ ] for each OS connected to a client, it has to be connected to at least one DISS and one REPS
            [ ] a DISS must be compatible with the OS of the client = triangle (edges) between client-DISS-OS
            [ ] a REPS must be compatible with the OS of its DISS = triangle (edges) between REPS-DISS-OS
            [ ] for each client, there must be a path client->DISS->REPS, otherwise, the client won't get updates*
            [ ] each OS must be connected to exactly one "become_client", one "become_diss" and one "become_reps" script
            [ ] a host can only have one role at the time. It can not exist in multiple node collections (ArangoDB allows it)
    */

    /* If one of the items is wrong, prompt the administrator and ask to remediate the situation (propose a solution when possible) */

    Ok(())
}

pub async fn db_create_update_database(db_info: &DatabaseInfo) -> Result<()> {
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

    match db_connection
        .create_database(db_info.db_name.as_str())
        .await
    {
        Ok(_) => {
            println!(
                "- database \'{}\' : {} - created",
                db_info.db_name,
                "Ok".green().bold()
            );
        }
        Err(e) => {
            if format!("{:?}", e).contains("duplicate database name") {
                println!(
                    "- database \'{}\' : {} - already exists",
                    db_info.db_name,
                    "OK".green().bold()
                );
            } else {
                println!(
                    "- database \'{}\' : {} - problem encountered in creating database",
                    db_info.db_name,
                    "NOK".red().bold()
                );
                println!("{:?}", e);
            }
        }
    }
    Ok(())
}

pub async fn db_create_update_node_collection(
    db_info: &DatabaseInfo,
    collection_name: &str,
) -> Result<()> {
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

    match db.create_collection(collection_name).await {
        Ok(_) => {
            println!(
                "- collection \'{}\' : {} - created",
                collection_name,
                "Ok".green().bold()
            );
        }
        Err(e) => {
            if format!("{:?}", e).contains("duplicate name") {
                println!(
                    "- collection \'{}\' : {} - already exists",
                    collection_name,
                    "OK".green().bold()
                );
            } else {
                println!(
                    "- collection \'{}\' : {} - problem encountered when creating collection",
                    collection_name,
                    "NOK".red().bold()
                );
                println!("{:?}", e);
            }
        }
    }

    Ok(())
}

pub async fn db_create_update_edge_collection(
    db_info: &DatabaseInfo,
    collection_name: &str,
) -> Result<()> {
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

    match db.create_edge_collection(collection_name).await {
        Ok(_) => {
            println!(
                "- collection \'{}\' : {} - created",
                collection_name,
                "Ok".green().bold()
            );
        }
        Err(e) => {
            if format!("{:?}", e).contains("duplicate name") {
                println!(
                    "- collection \'{}\' : {} - already existing",
                    collection_name,
                    "OK".green().bold()
                );
            } else {
                println!(
                    "- collection \'{}\' : {} - problem encountered when creating collection",
                    collection_name,
                    "NOK".red().bold()
                );
                println!("{:?}", e);
            }
        }
    }

    Ok(())
}

pub async fn db_create_update_client(db_info: &DatabaseInfo, host_info: &NodeClient) -> Result<()> {
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
        r#"UPSERT {{ "_key": "{}" }} INSERT {{"hostname": "{}", "ip": "{}", "osname": "{}", "osversion": "{}", "_key": "{}" }} REPLACE {{"hostname": "{}", "ip": "{}", "osname": "{}", "osversion": "{}", "_key": "{}" }} IN clients"#,
        host_info._key,
        host_info.hostname,
        host_info.ip,
        host_info.osname,
        host_info.osversion,
        host_info._key,
        host_info.hostname,
        host_info.ip,
        host_info.osname,
        host_info.osversion,
        host_info._key
    );

    let _: Vec<serde_json::Value> = match db.aql_str(client_creation_query.as_str()).await {
        Ok(content) => {
            println!("{}", "Client added/updated in database".bold().blue());
            content
        }
        Err(e) => {
            println!("{}", "Problem encountered with AQL query".red());
            println!("{:?}", e);
            vec![json!([""])]
        }
    };
    Ok(())
}

pub async fn db_create_update_diss(db_info: &DatabaseInfo, host_info: NodeDiss) -> Result<()> {
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
        r#"UPSERT {{ "_key": "{}" }} INSERT {{"hostname": "{}", "ip": "{}", "osname": "{}", "osversion": "{}", "_key": "{}" }} REPLACE {{"hostname": "{}", "ip": "{}", "osname": "{}", "osversion": "{}", "_key": "{}" }} IN diss"#,
        host_info._key,
        host_info.hostname,
        host_info.ip,
        host_info.osname,
        host_info.osversion,
        host_info._key,
        host_info.hostname,
        host_info.ip,
        host_info.osname,
        host_info.osversion,
        host_info._key
    );

    let _: Vec<serde_json::Value> = match db.aql_str(diss_creation_query.as_str()).await {
        Ok(content) => {
            println!("{}", "DISS added/updated in database".bold().blue());
            content
        }
        Err(e) => {
            println!("{}", "Problem encountered with AQL query".red());
            println!("{:?}", e);
            vec![json!([""])]
        }
    };
    Ok(())
}

pub async fn db_create_update_reps(db_info: &DatabaseInfo, host_info: NodeReps) -> Result<()> {
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
        r#"UPSERT {{ "_key": "{}" }} INSERT {{"hostname": "{}", "ip": "{}", "osname": "{}", "osversion": "{}", "_key": "{}" }} REPLACE {{"hostname": "{}", "ip": "{}", "osname": "{}", "osversion": "{}", "_key": "{}" }} IN reps"#,
        host_info._key,
        host_info.hostname,
        host_info.ip,
        host_info.osname,
        host_info.osversion,
        host_info._key,
        host_info.hostname,
        host_info.ip,
        host_info.osname,
        host_info.osversion,
        host_info._key
    );

    let _: Vec<serde_json::Value> = match db.aql_str(reps_creation_query.as_str()).await {
        Ok(content) => {
            println!("{}", "REPS added/updated in database".bold().blue());
            content
        }
        Err(e) => {
            println!("{}", "Problem encountered with AQL query".red());
            println!("{:?}", e);
            vec![json!([""])]
        }
    };
    Ok(())
}

pub async fn db_create_update_os(db_info: &DatabaseInfo, req: &NodeHostRequest) -> Result<()> {
    // Parsing repositories ressource file
    let config_builder = Config::builder()
        .add_source(File::new("repo-sources.yml", FileFormat::Yaml))
        .build()
        .unwrap();
    let repo_source = config_builder.try_deserialize::<RepoSource>().unwrap();

    // Check that the client's OS is supported by the repositories ressource file
    // If the OS is supported, grab a copy of its info -> useful for the next step
    let mut host_os = Os {
        _key: "".to_string(),
        osname: "".to_string(),
        osversion: "".to_string(),
        repositories: vec!["".to_string()],
    };

    let mut host_is_supported = false;
    // The following match needs to be rewritten in the future to avoid code repetition.
    // --> Make use of generic types ?
    match req {
        NodeHostRequest::Client(host_info) => {
            for supported_os in repo_source.list.into_iter() {
                if (host_info.osname == supported_os.osname)
                    && (host_info.osversion == supported_os.osversion)
                {
                    host_is_supported = true;
                    host_os = supported_os;
                }
            }
            if !host_is_supported {
                // OS not supported by the tool
                println!(
                    "{}",
                    format!(
                        "{} {} is not supported by the tool",
                        host_info.osname, host_info.osversion
                    )
                    .red()
                );
                return Err(ReadlineError::Interrupted);
            }
        }
        NodeHostRequest::Diss(host_info) => {
            for supported_os in repo_source.list.into_iter() {
                if (host_info.osname == supported_os.osname)
                    && (host_info.osversion == supported_os.osversion)
                {
                    host_is_supported = true;
                    host_os = supported_os;
                }
            }
            if !host_is_supported {
                // OS not supported by the tool
                println!(
                    "{}",
                    format!(
                        "{} {} is not supported by the tool",
                        host_info.osname, host_info.osversion
                    )
                    .red()
                );
                return Err(ReadlineError::Interrupted);
            }
        }
        NodeHostRequest::Reps(host_info) => {
            for supported_os in repo_source.list.into_iter() {
                if (host_info.osname == supported_os.osname)
                    && (host_info.osversion == supported_os.osversion)
                {
                    host_is_supported = true;
                    host_os = supported_os;
                }
            }
            if !host_is_supported {
                // OS not supported by the tool
                println!(
                    "{}",
                    format!(
                        "{} {} is not supported by the tool",
                        host_info.osname, host_info.osversion
                    )
                    .red()
                );
                return Err(ReadlineError::Interrupted);
            }
        }
    }

    // Create/update the OS in the database
    // The repo list has to look like an array ["REPO 1", "REPO 2"] so a little formatting
    // is needed before sending the AQL query.
    let mut repo_list = String::new();
    for repo in host_os.repositories.into_iter() {
        repo_list.push_str(format!("\"{}\", ", repo).as_str());
    }

    let os_creation_query = format!(
        r#"UPSERT {{ "_key": "{}" }} INSERT {{ "_key": "{}", "osname": "{}", "osversion": "{}", "repositories": [{}] }} UPDATE {{ "repositories": [{}] }} IN os"#,
        host_os._key, host_os._key, host_os.osname, host_os.osversion, repo_list, repo_list
    );

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

    let _: Vec<serde_json::Value> = match db.aql_str(&os_creation_query.as_str()).await {
        Ok(content) => {
            println!("{}", "OS added/updated in database".bold().blue());
            content
        }
        Err(e) => {
            println!("{}", "Problem encountered with AQL query".red());
            println!("{:?}", e);
            vec![json!([""])]
        }
    };

    // Creation of scripts associated to the os

    let _ = db_create_update_script(db_info, host_os._key.clone()).await;

    // Create/update the link between client and OS
    // OS : host_os._key
    // Host : host_info._key

    let mut edge_creation_query = String::new();

    match req {
        NodeHostRequest::Client(host_info) => {
            edge_creation_query = format!(
                r#"UPSERT {{ "_from": "clients/{}", "_to": "os/{}" }} INSERT {{ "_from": "clients/{}", "_to": "os/{}" }} UPDATE {{ }} IN uses_os"#,
                host_info._key, host_os._key, host_info._key, host_os._key,
            );
        }
        NodeHostRequest::Diss(host_info) => {
            edge_creation_query = format!(
                r#"UPSERT {{ "_from": "diss/{}", "_to": "os/{}" }} INSERT {{ "_from": "diss/{}", "_to": "os/{}" }} UPDATE {{ }} IN uses_os"#,
                host_info._key, host_os._key, host_info._key, host_os._key,
            );
        }
        NodeHostRequest::Reps(host_info) => {
            edge_creation_query = format!(
                r#"UPSERT {{ "_from": "reps/{}", "_to": "os/{}" }} INSERT {{ "_from": "reps/{}", "_to": "os/{}" }} UPDATE {{ }} IN uses_os"#,
                host_info._key, host_os._key, host_info._key, host_os._key,
            );
        }
    }

    let _: Vec<serde_json::Value> = match db.aql_str(&edge_creation_query.as_str()).await {
        Ok(content) => {
            println!(
                "{}",
                "Edge between host and os added/updated in database"
                    .bold()
                    .blue()
            );
            content
        }
        Err(e) => {
            println!("{}", "Problem encountered with AQL query".red());
            println!("{:?}", e);
            vec![json!([""])]
        }
    };

    Ok(())
}

pub async fn db_create_update_script(db_info: &DatabaseInfo, os: String) -> Result<()> {

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

    // Creation of script documents and associated edges (script --> os) in database
    for role in ["client", "diss", "reps"] {
        match get_script_from_source_file(role, os.as_str()) {
            Ok(script) => {
                // The os compatible list has to look like an array ["hash 1", "hash 2"] so a little formatting
                // is needed before sending the AQL query.
                
                let mut hash_os_list = String::new();
                for os_tmp in script.compatible_with.clone().into_iter() {
                    hash_os_list.push_str(format!("\"{}\", ", os_tmp).as_str());
                }

                // Increment database
                let script_creation_query = format!(
                    r#"UPSERT {{ "_key": "{}" }} INSERT {{ "_key": "{}", "role": "{}", "content": "{}", "compatible_with": [{}] }} UPDATE {{ "content": "{}", "compatible_with": [{}] }} IN scripts"#,
                    script._key,
                    script._key,
                    script.role,
                    script.content,
                    hash_os_list,
                    script.content,
                    hash_os_list
                );
                        
                let _: Vec<serde_json::Value> = match db.aql_str(&script_creation_query.as_str()).await {
                    Ok(content) => {
                        println!("{}", format!("Script (role {}) added/updated in database", role).bold().blue());
                        content
                    }
                    Err(e) => {
                        println!("{}", "Problem encountered with AQL query".red());
                        println!("{:?}", e);
                        vec![json!([""])]
                    }
                };

                // Creation of edges between the os and its associated scripts
                // Also, since a script can be associated to multiple os,
                // the script is going to be linked to every compatible os
                // already existing in the database
                println!("{}", "    ↪ Script linked to os".bold().blue());
                for compatible_os in script.compatible_with.into_iter() {
                    let edge_creation_query = format!(
                        r#"UPSERT {{ "_from": "scripts/{}", "_to": "os/{}" }} INSERT {{ "_from": "scripts/{}", "_to": "os/{}" }} UPDATE {{ }} IN script_compatible_with"#,
                        script._key,
                        compatible_os,
                        script._key,
                        compatible_os
                    );
                    // This query creates the "script_compatible_with" link even if the os is not present in the database.
                    // The others functions are not affected by this edge existing for nothing (at the moment) and, if this
                    // os shows up later in the database, the link will already be there.

                    let _: Vec<serde_json::Value> = match db.aql_str(&edge_creation_query.as_str()).await {
                        Ok(content) => {
                            println!("{}", "        ↪ Script linked to a compatible os".bold().blue());
                            content
                        }
                        Err(e) => {
                            println!("{}", "Problem encountered with AQL query".red());
                            println!("{:?}", e);
                            vec![json!([""])]
                        }
                    };
                }

                


            }
            Err(e) => {
                println!("Role {} : {}", role, e);
            }
        }
    }

    Ok(())
}

pub async fn db_create_graph(db_info: &DatabaseInfo)  -> Result<()> {

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

    let handles_edge = EdgeDefinition {
        collection: "handles".to_string(),
        from: vec!["diss".to_string()],
        to: vec!["clients".to_string()]
    };

    let redistributes_to_edge = EdgeDefinition {
        collection: "redistributes_to".to_string(),
        from: vec!["reps".to_string()],
        to: vec!["diss".to_string()]
    };

    let my_hosts_graph = Graph::builder()
        .name("all_my_hosts".to_string())
        .edge_definitions(vec![handles_edge, redistributes_to_edge])
        .orphan_collections(vec!["reps".to_string(), "diss".to_string(), "clients".to_string()])
        .build();

    match db.create_graph(my_hosts_graph, true).await {
        Ok(_) => {
            println!(
                "- graph all_my_hosts : {} - created",
                "Ok".green().bold()
            );
        }
        Err(e) => {
            
            if format!("{:?}", e).contains("graph already exists") {
                println!(
                    "- graph all_my_hosts : {} - already existing",
                    "OK".green().bold()
                );
            } else {
                println!(
                    "- graph all_my_hosts : {} - problem encountered whent creating graph",
                        "NOK".red().bold()
                );
                println!("{:?}", e);
            }
        }
    };

    Ok(())
}

pub fn launch_webgui(db_info: &DatabaseInfo) {

    println!("Launching native web gui of ArangoDB");

    println!("To graphically edit your future topoly and create your flows between hosts,");
    println!("after login, go to {} and activate \"Load full graph\" option.", "GRAPHS/all_my_hosts".bold().green());
    println!("There you can create appropriate edges between hosts.");
    println!("");
    println!("Direct link to graph (login still required) :");
    println!("{}",
        format!("http://{}:{}/_db/karnaboo/_admin/aardvark/index.html#graphs-v2/all_my_hosts", db_info.arangodb_server_address, db_info.arangodb_server_port)
            .bold()
            .green()
    );
    println!("");
    
    thread::sleep(Duration::from_secs(1));

    let _ = Command::new("xdg-open")
        .arg(format!("http://{}:{}", db_info.arangodb_server_address, db_info.arangodb_server_port))
        .spawn()
        .expect("Unable to launch web gui of ArangoDB");
}

/* ============================================================================
========================== Types declarations =================================
 ============================================================================== */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeClient {
    pub hostname: String,
    pub ip: String,
    pub osname: String,
    pub osversion: String,
    pub _key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeReps {
    pub hostname: String,
    pub ip: String,
    pub osname: String,
    pub osversion: String,
    pub _key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeDiss {
    pub hostname: String,
    pub ip: String,
    pub osname: String,
    pub osversion: String,
    pub _key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Deserialize)]
struct RepoSource {
    list: Vec<Os>,
}

#[derive(Deserialize)]
struct Os {
    _key: String,
    osname: String,
    osversion: String,
    repositories: Vec<String>,
}

