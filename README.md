# Karnaboo

Streamline your update flows in open source environments !
*(not finished yet)*

## Context and objective

As I wanted to learn Rust and programming in general, I set myself an overly ambitious goal to see where it takes me : build a tool to help administrators regulate and streamline the update flows of their GNU/Linux hosts across an infrastructure. Instead of having multiple hosts downloading the same upgrades and stressing the organisation's internet connectivity, one local mirror of the repositories is available for them, depending on their OS version and their position in the topology. **The goal here is to have all these update flows and configurations managed in a centralized way.** Such a system has been around for a long time in "non-free environments" and it probably already exists for open source infrastructures as well, but, since my goal here is to learn by doing, I voluntarily didn't search for it and just started coding (not reinventing "apt-mirror" here. On the contrary, we use it and try to build upon it).

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

### What is functioning so far

- direct interaction with the database : on karnaboo server, you can directly enter AQL queries and see the database response, giving you full control of the database
- agent is able to send a request through the network
- server can receive and buffer multiple requests and let the administrator (user) decide to agree or drop each one

## TO-DO list
(not in order of priority)

*** Server side ***
- [ ] add autocompletion
- [ ] function to create own database from scratch in a working ArangoDB instance
- [ ] function to check database consistency
- [ ] fill "os" nodes with the actual repositories -> hardcoded list ? files available online through gitlab ?
- [ ] handling configuration file : present at the root directory of the program or path specified as a command line argument
- [ ] improve error handling and stability by getting rid of all "unwrap" and "expect" methods
- [ ] add an Arc<Mutex<T>> to make sure the database is accessed in a regulated way

*** Agent side ***
- [ ] pass arguments in the command line (no need for a whole CLI at the moment, maybe later) : server address and future role of the host ("./karnaboo-agent 10.23.1.2 client")
- [ ] get local system informations to send real requests
- [ ] functions to make the local system act accordingly to its new role (change repositories, perform a mirroring of remote repositories...)

*** Others ***
- [ ] visuals for the "Principles" and "Usage" sections
- [ ] comment the code
- [ ] begin user documentation
- [ ] encrypt communications

## Contributing
All contributions, tips and ideas are more than welcome.

## Authors and acknowledgment
TBD

## License
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
