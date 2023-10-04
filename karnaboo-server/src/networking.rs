/*
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use crate::configuration::Networking;
use crate::database::{self, NodeClient, NodeHostRequest};
use arangors::uclient::reqwest::ReqwestClient;
use arangors::Database;
use colored::Colorize;
use futures::lock::Mutex;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::sync::Arc;

pub async fn thread_networking(
    networking_info: Networking,
    waiting_requests_buffer_networking: Arc<Mutex<Vec<database::NodeHostRequest>>>,
) {
    let socket_address = format!(
        "{}:{}",
        networking_info.server_address, networking_info.server_port
    );

    let listener: TcpListener = match TcpListener::bind(&socket_address) {
        Ok(open_socket) => open_socket,
        Err(e) => {
            println!(
                "{}",
                format!("Fatal error : impossible to open socket : {:?}", e)
                    .bold()
                    .red()
            );
            println!(
                "{}",
                "Please fix the problem and relaunch the program."
                    .bold()
                    .red()
            );
            std::process::exit(0);
        }
    };

    // Buffer for hosts requests not answered yet
    let mut request_handler: TcpStream;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                request_handler = stream;

                let mut buffer: [u8; 1024] = [0; 1024];
                let size = request_handler
                    .read(&mut buffer)
                    .expect("Unable to read from TcpStream");
                let serialized_request_content = String::from_utf8_lossy(&buffer[..size]);
                let deserialized_request_content: database::NodeHostRequest =
                    serde_json::from_str(&serialized_request_content)
                        .expect("Unable to deserialize data received from TcpStream");

                // Inform user in real time about new request
                match &deserialized_request_content {
                    NodeHostRequest::Client(content) => {
                        println!(
                            "{}",
                            format!(
                                "New request for client role from \'{}\' at {}",
                                content.hostname, content.ip
                            )
                            .bold()
                            .blue()
                        );
                    }
                    NodeHostRequest::Diss(content) => {
                        println!(
                            "{}",
                            format!(
                                "New request for DISS role from \'{}\' at {}",
                                content.hostname, content.ip
                            )
                            .bold()
                            .blue()
                        );
                    }
                    NodeHostRequest::Reps(content) => {
                        println!(
                            "{}",
                            format!(
                                "New request for REPS role from \'{}\' at {}",
                                content.hostname, content.ip
                            )
                            .bold()
                            .blue()
                        );
                    }
                }

                // Add this request to the waiting requests buffer
                let mut waiting_requests = waiting_requests_buffer_networking.lock().await;
                waiting_requests.push(deserialized_request_content);
            }
            Err(e) => {
                eprintln!("Unable to establish TcpStream with remote host : {}", e);
            }
        }
    }
}
