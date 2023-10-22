/*
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use config::{self, Config, File, FileFormat};
use configuration::command_line_parsing;
use futures::lock::Mutex;
use std::sync::Arc;
use std::env;

mod cli;
mod commands;
mod configuration;
mod database;
mod networking;
mod enforce;

fn main() {
    // Tokio runtime necessary for database access through async http
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Parsing configuration file
    let args: Vec<String> = env::args().collect();

    let user_arguments = command_line_parsing(args);

    configuration::check_user_arguments(&user_arguments);

    let config_builder = Config::builder()
        .add_source(File::new(user_arguments.config_file_path.as_str(), FileFormat::Ini))
        .build()
        .unwrap();
    let user_config = config_builder
        .try_deserialize::<configuration::UserConfig>()
        .unwrap();

    // Shared ressources between threads
    // Buffer for hosts requests not answered yet - used by CLI and Networking threads
    let waiting_requests_buffer: Arc<Mutex<Vec<database::NodeHostRequest>>> =
        Arc::new(Mutex::new(vec![]));
    let waiting_requests_buffer_cli = waiting_requests_buffer.clone();
    let waiting_requests_buffer_networking = waiting_requests_buffer.clone();

    // Networking
    let networking_info_for_net_thread = user_config.networking.clone();
    let networking_info_for_cli_thread = user_config.networking.clone();
    // let networking_thread_handler =
    //     thread::spawn(move || networking::thread_networking(networking_info, waiting_requests_buffer_networking));
    let networking_thread_handler = rt.spawn(networking::thread_networking(
        networking_info_for_net_thread,
        waiting_requests_buffer_networking,
    ));

    //CLI
    let cli_thread_handler = rt.spawn(cli::thread_cli(
        user_config.databaseinfo,
        waiting_requests_buffer_cli,
        networking_info_for_cli_thread,
        user_arguments.repo_sources_path,
        user_arguments.script_bank_path
    ));

    // Database (thread not useful at the moment)
    // let db_thread_handler = rt.spawn(database::thread_database_interact());

    // Wait for threads to finish
    let _ = rt.block_on(networking_thread_handler);
    let _ = rt.block_on(cli_thread_handler);
    // let _ = rt.block_on(db_thread_handler);
}
