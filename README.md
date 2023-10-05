# Karnaboo

Streamline your update flows in open source environments !

## Context and description

As I wanted to learn Rust and programming, I set myself an overly ambitious goal to see where it takes me. I decided to build a tool to help administrators regulate and streamline the update flows of their GNU/Linux hosts across an infrastructure. Instead of having multiple hosts downloading the same upgrades and stressing the organisation's internet connectivity, one local mirror of the repositories is available for them, depending on their distribution version and their position in the topology.

I am well aware that such a system has existed for a long time for "non-free environments". Also, it probably already exists for open source infrastructures as well and I am just not aware of it, but, since my goal here is to learn Rust (and its environment), I voluntarily didn't search for it and just started writing it myself.

I am releasing this tool as an open source project for the following reasons :
- it might help someone somewhere someday
- maybe some sysadmin has ideas for improving this tool and would be willing to talk about it
- hopefully some contributors may be interested in helping to develop this tool and make it evolve into something bigger, more secure and actually useful for organizations

## Usage
1. Fill the database with every host
2. Organize virtually your update flows by linking nodes in the database
3. Enforce your virtual structure through the server/agents

## Visuals
TBD

## Installation
TBD

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
All contributions and ideas are more than welcome.

## Authors and acknowledgment
TBD

## License
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
