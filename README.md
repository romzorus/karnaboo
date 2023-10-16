# Karnaboo

Streamline your update flows in open source environments !

## Context and objective

The goal of this tool is to let administrators regulate and streamline **in a centralized and visual way** the update flows of their GNU/Linux hosts across an infrastructure. Also, in the long term, we want this tool to handle as much packaging systems and distributions as possible (apt, dnf, snap...).

We wishfully hope that :
- it might help someone somewhere someday,
- maybe a sysadmin passing by could share ideas to improve this tool based on his actual experience,
- a few contributors may be interested in helping and making this tool more reliable and more secure.

## Principles

### Different roles
- "REPS" : REPatriation Server -> located in your DMZ and actually getting the updates from the official repositories
- "DISS" : DIStribution Server -> located in your LAN, close to your clients, getting the updates from the REPS and making it available for the clients
- Client : local host requiring its updates

### Where Karnaboo comes in
The REPS, DISS and clients all have the Karnaboo agent installed.

In parallel, you have a Karnaboo server with its database running somewhere (where it can be reached by all the agents).
After registering each host in the database, the Karnaboo server will then tell each machine what to do (where to look for updates...etc) according to the topology you decided in the database.

## Installation
There are no packages yet so you have to build the tool yourself. However, Rust (Cargo) makes it really easy for us.

### Karnaboo server
*** Prerequisites ***
1. a functioning local [Rust installation](https://www.rust-lang.org/fr/tools/install)
2. a functioning [ArangoDB instance](https://arangodb.com/download-major/)
3. `# apt install build-essential pkg-config libssl-dev`

*** Building ***
```
git clone https://gitlab.com/romzorus/karnaboo.git
cd karnaboo/karnaboo-server
cargo build --release
```
Now your executable **karnaboo-server** is in the `target/release` folder. You can grab the file, place it anywhere and juste execute it with `./karnaboo-server` (the config/repo-sources.yml/script_bank.yml will need to be in the same folder though) or you can just stick to Rust and use `cargo run`in the `karnaboo/karnaboo-server` folder.

*** Karnaboo agent ***
```
git clone https://gitlab.com/romzorus/karnaboo.git
cd karnaboo/karnaboo-agent
cargo build --release
```

Now your executable **karnaboo-agent** is in the `target/release` folder. You can grab the file, place it anywhere and juste execute it with `./karnaboo-agent [arguments]` or you can just stick to Rust and use `cargo run -- [arguments]`in the `karnaboo/karnaboo-agent` folder.

## Usage

1. Fill the database with every host ("push" mode : the server listens while the agents send requests to be taken into account and to receive instructions according to the role they want to have and their position in the topology)
2. Organize your update flows by linking nodes in the database
3. Enforce your virtual structure through the server/agents

## What is already working

- direct interaction with the database : on karnaboo server, you can directly enter AQL queries and see the database response, giving you control over the data (AQL can't let you create or manage database and collections but only their content)
- after a fresh ArangoDB installation, the server can create in it everything it needs and begin to wait for registration requests
- once your database reflects what you have (the nodes) and what you want (the edges), you can enforce it on compatible distributions

### Compatible distributions (as of 16/10/2023)

All the following distributions have been successfully tested as clients, DISS and REPS :

- Linux Mint 21
- Ubuntu 22.04 (Desktop and Server)
- Ubuntu 23.04 (Desktop and Server)
- Ubuntu 23.10 (Desktop and Server)

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
- [ ] establish a communication protocol between server and agent so that the communication doesn't have to use multiple ports
- [ ] create installation script / packaging .deb and more

## Contributing
All contributions, tips and ideas are more than welcome.

## Authors and acknowledgment
TBD

## License
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
