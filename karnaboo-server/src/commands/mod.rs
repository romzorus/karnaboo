/*
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use colored::Colorize;
use std::io::{stdout, Write};

pub fn help() {
    println!("");
    println!("So far, the following commands are implemented :");
    println!("");
    println!("Command °° Description °° Alternative or Exemple");
    println!("--------||-------------||------------------------");
    println!(
        "{}    °° Terminates silvadmin process °° ex | quit | q",
        "exit".bold().white()
    );
    println!(
        "{}  °° Informations about the master server °° stat",
        "status".bold().white()
    );
    println!(
        "{}    °° Show this help message °° h | ?",
        "help".bold().white()
    );
    println!(
        "{}    °° Go to database interaction mode °° for db",
        "db".bold().white()
    );
    println!(
        "{}    °° Answer hosts requests to be handled °° ansreq | ar",
        "answer request".bold().white()
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
    println!(r"");
}

pub fn show_goodbye_message() {
    println!("");
    println!(r" ██                        Ending program now...                   ██");
    println!(r" █████                          See you !                       █████");
    println!(r" ████████████████████████████████████████████████████████████████████");
}

pub fn yes_or_no_question() -> bool {
    let answer: bool;
    let mut user_input = String::with_capacity(3);

    loop {
        print!("Yes or No ? (default: yes) ");
        let _ = stdout().flush();

        user_input.clear();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read answer");
        let user_input = user_input.trim();

        if user_input.is_empty() {
            answer = true;
            break;
        } else if ["Yes", "yes", "Y", "y"].contains(&user_input) {
            answer = true;
            break;
        } else if ["No", "no", "N", "n"].contains(&user_input) {
            answer = false;
            break;
        } else {
            println!("{}", "Invalid answer".bold().red());
        }
    }
    answer
}
