# Karnaboo

Streamline your update flows in open source environments !

## Context and description

As I wanted to learn Rust and programming in general, I set myself an overly ambitious goal to see where it takes me : build a tool to help administrators regulate and streamline the update flows of their GNU/Linux hosts across an infrastructure. Instead of having multiple hosts downloading the same upgrades and stressing the organisation's internet connectivity, one local mirror of the repositories is available for them, depending on their OS version and their position in the topology. All these update flows are managed in a centralized way. Such a system has been around for a long time in "non-free environments" and it probably already exists for open source infrastructures as well, but, since my goal here is to learn by doing, I voluntarily didn't search for it and just started coding (not reinventing "apt-mirror" here. On the contrary, we use it and try to build upon it).

I am releasing this tool as an open source project for the following reasons :
- it might help someone somewhere someday
- maybe a sysadmin passing by could get ideas to improve this tool
- hopefully a few contributors may be interested in helping and making this tool evolve into something bigger, more professional and more secure

## Usage

1. Fill the database with every host
2. Organize virtually your update flows by linking nodes in the database
3. Enforce your virtual structure through the server/agents

## Visuals
TBD

## Installation
### Prerequisites
- a functioning local Rust installation
- a functioning and reachable ArangoDB instance

### Building
*** Karnaboo server ***
1. clone the "karnaboo-server" repository
2. write your configuration file
3. Server building : go to "karnaboo-server" and "cargo build"

*** Karnaboo agent ***
1. clone the "karnaboo-agent" repository
3. Server building : go to "karnaboo-agent" and "cargo build"


## TO-DO list
(not in order of priority)

*** Server side ***
- [ ] add autocompletion
- [ ] function to create own database from scratch in a working ArangoDB instance
- [ ] function to check database consistency and coherence
- [ ] fill "os" nodes with the actual repositories -> hardcoded list ? files available online through gitlab ?
- [ ] handling configuration file : present at the root directory of the program or path specified as a command line argument
- [ ] improve error handling and stability by getting rid of all "unwrap" and "expect" methods

*** Agent side ***
- [ ] pass arguments in the command line (no need for a whole CLI at the moment, maybe later) : server address and future role of the host ("./karnaboo-agent 10.23.1.2 client")
- [ ] get local system informations to send real requests
- [ ] functions to make the local system act accordingly to its new role (change repositories, perform a mirroring of remote repositories...)


## Contributing
All contributions, tips and ideas are more than welcome.

## Authors and acknowledgment
TBD

## License
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
