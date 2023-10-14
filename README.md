# Karnaboo

Streamline your update flows in open source environments !
*(not finished yet)*

## Context and objective

As I wanted to learn Rust and programming in general, I set myself an overly ambitious goal to see where it takes me : build a tool to help administrators regulate and streamline the update flows of their GNU/Linux hosts across an infrastructure. Instead of having multiple hosts downloading the same upgrades and stressing the organisation's internet connectivity, one local mirror of the repositories is available for them, depending on their OS version and their position in the topology. **The goal here is to have all these update flows and configurations managed in a centralized way.** Such a system has been around for a long time in "non-free environments" and it probably already exists for open source infrastructures as well, but, since my goal here is to learn by doing, I voluntarily didn't search for it and just started coding (not reinventing "apt-cache" here. On the contrary, we use it and try to build upon it).

I am releasing this tool as an open source project for the following reasons :
- it might help someone somewhere someday
- maybe a sysadmin passing by could get ideas to improve this tool
- hopefully a few contributors may be interested in helping and making this tool evolve into something bigger, more professional and more secure

## Principles

### Different roles
- REPS : REPatriation Server -> located in your DMZ and actually getting the updates from the official repositories
- DISS : DIStribution Server -> located in your LAN, close to your clients, getting the updates from the REPS and making it available for the clients
- Client : local host requiring its updates

### Where Karnaboo comes in
The REPS, DISS and clients all have the Karnaboo agent installed.
On another machine, you have a Karnaboo server running where it can be reached by all the Karnaboo agents (and have access to a database).
The Karnaboo server will then tell each machine what to do (where to look for updates...etc) according to the topology you decided in the database.

## Installation
### Prerequisites
- a functioning local Rust installation
- a functioning and reachable ArangoDB instance

### Building
*** Karnaboo server ***
1. clone the "karnaboo-server" repository
2. write your configuration file
3. go to "karnaboo-server" and run "cargo build"

*** Karnaboo agent ***
1. clone the "karnaboo-agent" repository
3. go to "karnaboo-agent" and run "cargo build"

## Usage

1. Fill the database with every host ("push" mode : the server listens while the agents send requests to be taken into account and to receive instructions according to the role they want to have)
2. Organize virtually your update flows by linking nodes in the database
3. Enforce your virtual structure through the server/agents

### What is working so far

- server can create a database and the required collections from scratch
- direct interaction with the database : on karnaboo server, you can directly enter AQL queries and see the database response, giving you control over the data (AQL can't let you create or manage database and collections but only their content)
- agent can register with the server, receive a script from it, execute and send the return back to the server
- server can receive and buffer multiple registration requests and let the administrator (user) decide to register or not each host
- server can, based on the registration requests, fill the database with real client's informations and create appropriate edges and nodes
- server is beginning to enforce the roles : it checks each host in the database and send it the appropriate script for execution


## TO-DO list
(not in order of priority)

*** Server side ***
- [ ] add autocompletion
- [ ] function to check database consistency
- [ ] handling configuration file : present at the root directory of the program or path specified as a command line argument
- [ ] improve error handling and stability by getting rid of all "unwrap" and "expect" methods
- [ ] add an Arc<Mutex<T>> to make sure the database is accessed in a regulated way
- [ ] have a single connexion to the database and pass its reference to the functions (instead of having each function create its own connexion each time)
- [ ] add a functionality to ensure a host is only in one collection (ArangoDB allows documents to have the same _key if they are in different collections, meaning a host can appear as a client and as a DISS at the same time)
- [ ] function to update scripts in the database from the source file, so that it can be split from db_create_update_os() and used less systematically --> update scripts when specifically asked by the user in the CLI, when the scripts don't already exist and/or (versioning)
- [ ] add in the database an attribute to client/DISS/REPS to track the status of the host (script executed successfully, still in progress, failed)
- [ ] introduce multi-threading for registration requests handling so that multiple hosts can send requests at the same time
- [ ] introduce multi-threading for enforcement so that each thread can handle a host (sending instructions -> wait for return -> deal with return after)

*** Agent side ***
- [ ] solution to execute the script while the agent is closed (apt-mirror can take several hours to finish...), and when the agent is opened again, it can continue the job where it left it
- [ ] solution for the agent to show output in realtime on the host (difference in std::process::Command between .spawn() and .output()...etc)

*** Others ***
- [ ] reorganize the code in a proper way, split the big functions, gather functions in specific files, improve readability
- [ ] visuals for the "Principles" and "Usage" sections
- [ ] comment the code
- [ ] begin user documentation
- [ ] encrypt communications
- [ ] actual scripts to implement role on supported OS
- [ ] establish a communication protocol between server and agent so that the communication doesn't have to use multiple ports
- [ ] create installation script

## Contributing
All contributions, tips and ideas are more than welcome.

## Authors and acknowledgment
TBD

## License
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
