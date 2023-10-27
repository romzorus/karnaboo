# Karnaboo

Streamline your update flows in open source environments !

## Context and objective

The goal of this tool is to let administrators regulate and streamline **in a centralized and visual way** the update flows of their GNU/Linux hosts across an infrastructure. Also, in the long term, we want this tool to handle as much packaging systems and distributions as possible (apt, dnf, snap...).

## How does it work ?

[Here is a video for you !](https://www.youtube.com/watch?v=QUFpegW3hGQ)

### Concept
On each GNU/Linux host, you put the Karnaboo agent, ready to be executed with root privileges.
Then each agent sends a registration request to the Karnaboo server which fills a graph database ([ArangoDB](https://arangodb.com/)) with host's informations.
After the database is complete, you go from *what you have* (the nodes) to *what you want* (the edges) by creating
links between hosts in the database. *This host will get its updates from this one now.*
When your future topology is complete and consistent, you just tell the Karnaboo server to enforce it.
The server sends specific instructions to each host depending on its position in the topology and the
role it is supposed to play now. Each host abides and reports when the adaptation is over.

### Different roles
* **Karnaboo server**
* *Through Karnaboo agent*
    - **REPS** (REPatriation Server) : located in your DMZ and actually getting the updates from the official repositories
    - **DISS** (DIStribution Server) : located in your LAN, close to your clients, getting the updates from the REPS and making it available for the clients
    - **Client** : local host requiring its updates (it can be a user terminal, a web server or anything else)

## Quickly try it with Docker

To easily test the Karnaboo server, install [Docker](https://docs.docker.com/get-docker/) then follow these steps :

1. Create an internal docker network to allow Karnaboo and ArangoDB to communicate
`docker network create --subnet=172.10.0.0/24 karnaboo-network`
2. Run ArangoDB
`docker run --net karnaboo-network --ip 172.10.0.50 -e ARANGO_ROOT_PASSWORD=arangodb -p 8529:8529 -d arangodb`
3. Check that tcp ports 9015 and 9016 are not blocked by your firewall (depends on your distribution)
4. Run the Karnaboo server
`docker run --net karnaboo-network --ip 172.10.0.51 -ti -p 9015:9015 -p 9016:9016 romzorus/karnaboo`
5. To graphically access the database when it is time to connect your nodes, open this URL in your browser : [http://127.0.0.1:8529](http://127.0.0.1:8529)

As for the agent, you still have to build and execute it on a host. There is no point in running an agent inside a container. It won't have all the necessary data and it won't act on the host accordingly.

## Installation

The tools themselves don't need to be *installed* indefinitely. The idea is to execute once, put everything in order, then leave your hosts alone. This isn't a supervision tool. Once they are correctly configured, your hosts leave their life on their own until you want to change everything again. In that case, you execute the server and the agents again.

However, the content of the database remains after use. This allows you to re-enforce in case you need to replace a machine or something else changes. Also, you can make your topology evolve without starting from scratch again. You just start the server, start the agents and make them wait for instructions, change what you need in the database and enforce again.

There are no packages yet so you have to build the tool yourself. However, Rust (Cargo) makes it really easy for us.

### Karnaboo server
*** Prerequisites ***
1. a functional local [Rust installation](https://www.rust-lang.org/fr/tools/install)
2. a functional [ArangoDB instance](https://arangodb.com/download-major/)
3. `sudo apt install build-essential pkg-config libssl-dev`
4. TCP ports 9015 and 9016 opened

*** Building ***
```
git clone https://gitlab.com/romzorus/karnaboo.git
cd karnaboo/karnaboo-server
cargo build --release
```
Now your executable **karnaboo-server** is in the `target/release` folder. You can grab the file, place it anywhere and just execute it with `./karnaboo-server` (the config/repo-sources.yml/script_bank.yml will need to be in the same folder though) or you can just stick to Rust and use `cargo run`in the `karnaboo/karnaboo-server` folder. **No need for root privileges.**

### Karnaboo agent
*** Prerequisites ***
1. a functional local [Rust installation](https://www.rust-lang.org/fr/tools/install)
2. TCP port 9017 opened

*** Building ***
```
git clone https://gitlab.com/romzorus/karnaboo.git
cd karnaboo/karnaboo-agent
cargo build --release
```

Now your executable **karnaboo-agent** is in the `target/release` folder. You can grab the file, place it anywhere and just execute it with `sudo ./karnaboo-agent [arguments]` or you can just stick to Rust and use `sudo cargo run -- [arguments]`in the `karnaboo/karnaboo-agent` folder. **Root privileges required.**

If the architectures allow it, you can also build your agent on the Karnaboo server, push the resulting file (the agent) on the client host (ssh, USB key, any other way) and execute it there.

## Usage
TBD

## What is already working

- direct interaction with the database : on Karnaboo server, you can directly enter AQL queries and see the database response, giving you control over the data (AQL can't let you create or manage database and collections but only their content)
- after a fresh ArangoDB installation, the server can create in it everything it needs and begin to wait for registration requests
- once your database reflects what you have (the nodes) and what you want (the edges), you can enforce it on compatible distributions

### Compatible distributions (as of 17/10/2023)

All the following distributions have been successfully tested as clients, DISS and REPS :

- Linux Mint 21
- Ubuntu 22.04 (Desktop and Server)
- Ubuntu 23.04 (Desktop and Server)
- Ubuntu 23.10 (Desktop and Server)
- Debian 12
- CentOS Stream 9

## TO-DO list
(not in order of priority)

*** Server side ***
- [ ] add autocompletion
- [ ] function to check database consistency
- [ ] improve error handling and stability by getting rid of all "unwrap" and "expect" methods
- [ ] add an Arc<Mutex<T>> to make sure the database is accessed in a regulated way
- [ ] have a single connexion to the database and pass its reference to the functions (instead of having each function create its own connexion each time)
- [ ] add a functionality to ensure a host is only in one collection (ArangoDB allows documents to have the same _key if they are in different collections, meaning a host can appear as a client and as a DISS at the same time)
- [ ] function to update scripts in the database from the source file, so that it can be split from db_create_update_os() and used less systematically --> update scripts when specifically asked by the user in the CLI, when the scripts don't already exist and/or (versioning)
- [ ] add in the database an attribute to client/DISS/REPS to track the status of the host (script executed successfully, still in progress, failed)
- [ ] introduce multi-threading for registration requests handling so that multiple hosts can send requests at the same time
- [ ] introduce multi-threading for enforcement so that each thread can handle a host (sending instructions -> wait for return -> deal with return after)
- [ ] add possibility to have clients directly handled by a REPS (small infrastructures without the need for DISS)

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
- [ ] add a little security in the communication protocol by using ip filtering (and expiring tokens issued by the agents for the server ?)
- [ ] CentOS compatibility : stop the fastestmirror process and reduce the repositories list to only a few, otherwise the packages are downloaded again when the fastest mirror changes

## Contributing
All contributions, tips and ideas are more than welcome.

We hope that :
- it might help someone somewhere someday,
- a sysadmin passing by could share ideas to improve this tool based on his actual experience,
- contributors may be interested in helping and making this tool more reliable, more universal and more secure.

## Authors and acknowledgment
TBD

## License
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
