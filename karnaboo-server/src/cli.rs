/*
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

/*
This module relates to the shell, its prompt, its notifications/messages and
its command interpreter. It checks the validity of a command then invokes the
corresponding function from the commands module.
*/
use crate::commands;
use crate::configuration::Networking;
use crate::enforce;
use crate::handlerequests;
use crate::{configuration::DatabaseInfo, database};

use colored::Colorize;
use futures::lock::Mutex;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;

pub async fn thread_cli(
    db_info: DatabaseInfo,
    waiting_requests_buffer_cli: Arc<Mutex<Vec<database::NodeHostRequest>>>,
    networking_info: Networking,
    repo_sources_path: String,
    script_bank_path: String,
) -> Result<()> {
    std::thread::sleep(Duration::from_secs(1));

    commands::show_welcome_message();

    let mut user_command: String;
    let mut rl = DefaultEditor::new()?;

    loop {
        // /*
        //     Interpréter et exécuter la commande --> en faire une fonction dédiée
        //     user_command_checker()
        //     user_command_exec()
        // */
        let readline = rl.readline(format!("{}", "Command $ ".bold()).as_str());

        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                user_command = line.clone();
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl-C
                commands::show_goodbye_message();
                exit(0);
            }
            Err(ReadlineError::Eof) => {
                // Ctrl-D
                commands::show_goodbye_message();
                exit(0);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                exit(1);
            }
        }

        // Command analysis
        let user_command_str = user_command.as_str();

        if user_command_str.is_empty() {
            continue;
        } else if ["exit", "ex", "quit", "q"].contains(&user_command_str) {
            commands::show_goodbye_message();
            exit(0);
        } else if ["status", "stat", "s"].contains(&user_command_str) {
            commands::status_info();
        } else if ["help", "h", "?"].contains(&user_command_str) {
            commands::help();
        } else if ["aqlmode", "aql"].contains(&user_command_str) {
            let return_db_cli = database::aql_mode(&db_info);
            let _ = return_db_cli.await;
        } else if ["dbbuild", "dbb"].contains(&user_command_str) {
            let return_db_check = database::db_build(&db_info);
            let _ = return_db_check.await;
        } else if ["dbcheck", "dbc"].contains(&user_command_str) {
            let return_db_check = database::db_check(&db_info);
            let _ = return_db_check.await;
        } else if ["dbgui", "dbg"].contains(&user_command_str) {
            commands::launch_webgui(&db_info);
        } else if ["answer request", "ansreq", "ar"].contains(&user_command_str) {
            let return_answer_request = handlerequests::answer_requests(
                &waiting_requests_buffer_cli,
                &db_info,
                &repo_sources_path,
                &script_bank_path,
            );
            let _ = return_answer_request.await;
        } else if ["enforce", "enf"].contains(&user_command_str) {
            let return_enforce = enforce::enforce(&db_info, &networking_info);
            let _ = return_enforce.await;
        } else {
            println!("{} : {}", "Invalid command".bold().red(), user_command_str);
        }

        user_command.clear();
    }
}
