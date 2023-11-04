/*
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use colored::Colorize;

pub fn help() {
    println!("");
    println!("So far, the following commands are implemented :");
    println!("");
    println!("Command °° Description °° Alternative or Exemple");
    println!("--------||-------------||------------------------");
    println!("{}    °° Show this help message °° h | ?", "help".bold());
    println!(
        "{}    °° Terminates karnaboo server process °° ex | quit | q",
        "exit".bold()
    );
    println!(
        "{}  °° Informations about this karnaboo server °° stat",
        "status".bold()
    );
    println!(
        "{}    °° Enter AQL queries and get results from the database °° aql",
        "aqlmode".bold()
    );
    println!(
        "{}    °° Build your database from scratch°° dbb",
        "dbbuild".bold()
    );
    println!(
        "{}    °° Answer hosts requests to be handled °° ansreq | ar",
        "answer request".bold()
    );
    println!(
        "{}    °° Launch native ArangoDB web GUI °° dbg",
        "dbgui".bold()
    );
    println!(
        "{}    °° Check the consistency of your database°° dbc",
        "dbcheck".bold()
    );
    println!(
        "{}    °° Enforce what you decided in the database to all your hosts °° enf",
        "enforce".bold()
    );
    println!("");
}

pub fn status_info() {
    println!("Informations about your server's status :");
    println!("...");
}

pub fn show_welcome_message() {
    println!(r" ████████████████████████████████████████████████████████████████████");
    println!(r" █████                                                          █████");
    println!(r" ██                           Welcome to                           ██");
    println!("");
    println!(r" ██╗  ██╗ █████╗ ██████╗ ███╗   ██╗ █████╗ ██████╗  ██████╗  ██████╗ ");
    println!(r" ██║ ██╔╝██╔══██╗██╔══██╗████╗  ██║██╔══██╗██╔══██╗██╔═══██╗██╔═══██╗");
    println!(r" █████╔╝ ███████║██████╔╝██╔██╗ ██║███████║██████╔╝██║   ██║██║   ██║");
    println!(r" ██╔═██╗ ██╔══██║██╔══██╗██║╚██╗██║██╔══██║██╔══██╗██║   ██║██║   ██║");
    println!(r" ██║  ██╗██║  ██║██║  ██║██║ ╚████║██║  ██║██████╔╝╚██████╔╝╚██████╔╝");
    println!(r" ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═════╝  ╚═════╝  ╚═════╝ ");
    println!(r"                                 https://gitlab.com/romzorus/karnaboo");
    println!("");
}

pub fn show_goodbye_message() {
    println!("");
    println!(r" ██                        Ending program now...                   ██");
    println!(r" █████                          See you !                       █████");
    println!(r" ████████████████████████████████████████████████████████████████████");
}
