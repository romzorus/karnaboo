/*
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use crate::configuration::Networking;
use crate::database::{self, NodeClient, NodeDiss, NodeHostRequest, NodeReps};
use colored::Colorize;
use futures::lock::Mutex;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

pub async fn thread_networking(
    networking_info: Networking,
    waiting_requests_buffer_networking: Arc<Mutex<Vec<database::NodeHostRequest>>>,
) {
    let socket_address = format!(
        "{}:{}",
        networking_info.server_address, networking_info.server_port
    );

    let listener: TcpListener = TcpListener::bind(&socket_address)
        .expect(format!("{}", "[Networking] unable to open socket".bold().red()).as_str());

    // Buffer for hosts requests not answered yet
    let mut request_handler: &TcpStream;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                request_handler = &stream;

                let remote_host_ip: String;
                match stream.peer_addr() {
                    Ok(remote_host_ip_tmp) => {
                        remote_host_ip = remote_host_ip_tmp.to_string();
                    }
                    Err(_) => {
                        println!("[Networking] unable to read remote host IP");
                        continue;
                    }
                }

                let mut buffer: [u8; 1024] = [0; 1024];
                let size: usize;
                match request_handler.read(&mut buffer) {
                    Ok(data_tmp) => {
                        size = data_tmp;
                    }
                    Err(_) => {
                        println!(
                            "[Networking] unable to bufferize data received from {}",
                            remote_host_ip
                        );
                        continue;
                    }
                }

                let serialized_request_content = String::from_utf8_lossy(&buffer[..size]);

                let deserialized_request_content: database::NodeHostRequest;
                match serde_json::from_str(&serialized_request_content) {
                    Ok(deserialized_data) => {
                        deserialized_request_content = deserialized_data;
                    }
                    Err(_) => {
                        println!(
                            "[Networking] unable to deserialize data received from {}",
                            remote_host_ip
                        );
                        continue;
                    }
                }

                /* It is necessary to deconstruct the received content, get the ip from TcpStream, then
                reconstruct into another NodeHostRequest, because, when karnaboo agent sends its request to
                the server, it doesn't know which network interface (meaning ip address) is going to be used
                at lower level (routing table and everything) so it can't put itself its own ip address in
                the request. The client's ip address has to be read from the outside aka the server.
                So, when the server receives a request from an ip address, it has to put itself this ip in
                the request in order for it to be complete. */

                // Complete the request with the actual ip address of the client
                // and alert user in real time about new request
                let final_request_content: database::NodeHostRequest;

                match &deserialized_request_content {
                    NodeHostRequest::Client(content) => {
                        println!(
                            "{}",
                            format!(
                                "New request for client role from \'{}\' at {}",
                                content.hostname, remote_host_ip
                            )
                            .bold()
                            .blue()
                        );

                        final_request_content = NodeHostRequest::Client(NodeClient {
                            hostname: content.hostname.clone(),
                            ip: remote_host_ip, // Getting ip address from TcpStream
                            osname: content.osname.clone(),
                            osversion: content.osversion.clone(),
                            _key: content._key.clone(),
                        });
                    }
                    NodeHostRequest::Diss(content) => {
                        println!(
                            "{}",
                            format!(
                                "New request for DISS role from \'{}\' at {}",
                                content.hostname, remote_host_ip
                            )
                            .bold()
                            .blue()
                        );

                        final_request_content = NodeHostRequest::Diss(NodeDiss {
                            hostname: content.hostname.clone(),
                            ip: remote_host_ip, // Getting ip address from TcpStream
                            osname: content.osname.clone(),
                            osversion: content.osversion.clone(),
                            _key: content._key.clone(),
                        });
                    }
                    NodeHostRequest::Reps(content) => {
                        println!(
                            "{}",
                            format!(
                                "New request for REPS role from \'{}\' at {}",
                                content.hostname, remote_host_ip
                            )
                            .bold()
                            .blue()
                        );

                        final_request_content = NodeHostRequest::Reps(NodeReps {
                            hostname: content.hostname.clone(),
                            ip: remote_host_ip, // Getting ip address from TcpStream
                            osname: content.osname.clone(),
                            osversion: content.osversion.clone(),
                            _key: content._key.clone(),
                        });
                    }
                }

                // Add this request to the waiting requests buffer
                let mut waiting_requests = waiting_requests_buffer_networking.lock().await;
                waiting_requests.push(final_request_content);
            }
            Err(e) => {
                eprintln!(
                    "[Networking] unable to establish TcpStream with remote host : {}",
                    e
                );
            }
        }
    }
}
