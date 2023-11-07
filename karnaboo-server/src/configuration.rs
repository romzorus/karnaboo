/*
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use serde::Deserialize;
use std::path::Path;
use std::process::exit;

use crate::commands::show_command_help_message;

pub fn command_line_parsing(args: Vec<String>) -> UserArguments {
    // Initialization of arguments
    // By default, all files are considered to be in a "config" folder located beside the executable.
    let mut user_arguments = UserArguments {
        config_file_path: "./config/karnaboo.conf".to_string(),
        repo_sources_path: "./config/repo-sources.yml".to_string(),
        script_bank_path: "./config/script_bank.yml".to_string(),
    };

    let tmp_args = args.clone();

    for (i, _arg) in tmp_args.into_iter().enumerate() {
        if ["-c", "--config", "--configuration"].contains(&args[i].as_str()) {
            // Looking for config file path
            // There needs to be a following argument...
            if args.len() >= (i + 2) {
                // (length=i+1) and we are looking for the next argument so (i+1+1) = (i+2)
                user_arguments.config_file_path = args[i + 1].clone();
            } else {
                // ... otherwise :
                show_command_help_message();
                break;
            }
        } else if ["-s", "--script"].contains(&args[i].as_str()) {
            // Looking for script bank file path
            // There needs to be a following argument...
            if args.len() >= (i + 2) {
                // (length=i+1) and we are looking for the next argument so (i+1+1) = (i+2)
                user_arguments.script_bank_path = args[i + 1].clone().to_lowercase();
            } else {
                show_command_help_message();
                break;
            }
        } else if ["-r", "--repo"].contains(&args[i].as_str()) {
            // Looking for repositories file path
            // There needs to be a following argument...
            if args.len() >= (i + 2) {
                // (length=i+1) and we are looking for the next argument so (i+1+1) = (i+2)
                user_arguments.repo_sources_path = args[i + 1].clone().to_lowercase();
            } else {
                show_command_help_message();
                break;
            }
        } else if ["-h", "--help"].contains(&args[i].as_str()) {
            show_command_help_message();
            break;
        }
    }

    user_arguments
}

pub fn check_user_arguments(user_arguments: &UserArguments) {
    if !Path::new(&user_arguments.config_file_path).exists() {
        println!(
            "Configuration file not found at {}. Abort.",
            user_arguments.config_file_path
        );
        exit(1);
    } else if !Path::new(&user_arguments.repo_sources_path).exists() {
        println!(
            "Repositories file not found at {}. Abort.",
            user_arguments.repo_sources_path
        );
        exit(1);
    } else if !Path::new(&user_arguments.script_bank_path).exists() {
        println!(
            "Script bank file not found at {}. Abort.",
            user_arguments.script_bank_path
        );
        exit(1);
    }
}

pub struct UserArguments {
    pub config_file_path: String,
    pub repo_sources_path: String,
    pub script_bank_path: String,
}

#[derive(Deserialize, Clone)]
pub struct UserConfig {
    pub databaseinfo: DatabaseInfo,
    pub networking: Networking,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseInfo {
    pub db_name: String,
    pub login: String,
    pub password: String,
    pub arangodb_server_address: String,
    pub arangodb_server_port: u16,
}

#[derive(Deserialize, Clone)]
pub struct Networking {
    pub server_address: String,
    pub server_port: u16,
}
