/*
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use serde::Deserialize;

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
