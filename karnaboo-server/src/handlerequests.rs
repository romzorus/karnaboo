/*
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use colored::Colorize;
use futures::lock::Mutex;
use std::sync::Arc;
use std::io::{stdout, Write};


use crate::configuration::DatabaseInfo;
use crate::database::{db_create_update_client, db_create_update_diss, db_create_update_reps, db_create_update_os};
use crate::database::NodeHostRequest;

pub async fn answer_requests(
    waiting_requests: &Arc<Mutex<Vec<NodeHostRequest>>>,
    db_info: &DatabaseInfo,
    repo_sources_path: &String,
    script_bank_path: &String
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
                    let return_os_client_creation = db_create_update_os(db_info, &req_clone_for_os, repo_sources_path, &script_bank_path);
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
                    let return_os_client_creation = db_create_update_os(db_info, &req_clone_for_os, repo_sources_path, &script_bank_path);
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
                    let return_os_client_creation = db_create_update_os(db_info, &req_clone_for_os, repo_sources_path, &script_bank_path);
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

fn yes_or_no_question(default: bool) -> bool {
    // Default value : yes = true and no = false
    let answer: bool;
    let mut user_input = String::with_capacity(3);

    loop {
        print!(
            "Yes or No ? (default: {}) ",
            if default { "yes" } else { "no" }
        );
        let _ = stdout().flush();

        let final_user_input: &str;
        user_input.clear();
        
        match std::io::stdin()
            .read_line(&mut user_input) {
                Ok(_) => {
                    final_user_input = user_input.trim();
                }
                Err(_) => {
                    final_user_input = "error";
                }
            }

        if final_user_input.is_empty() {
            answer = default;
            break;
        } else if ["Yes", "yes", "Y", "y"].contains(&final_user_input) {
            answer = true;
            break;
        } else if ["No", "no", "N", "n"].contains(&final_user_input) {
            answer = false;
            break;
        } else {
            println!("{}", "Invalid answer".bold().red());
        }
    }
    answer
}