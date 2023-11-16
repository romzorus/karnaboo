/*
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/
// #![feature(proc_macro_hygiene, decl_macro)]

extern crate tera;

use rocket::Rocket;
use rocket::Build;
use rocket::fs::NamedFile;
use rocket::response::content;
use rocket_dyn_templates::{Template, context};
use std::path::Path;

#[get("/")]
fn index() -> content::RawHtml<Template> {

    content::RawHtml(Template::render(
        "index",
        context! { username: "Romzorus", username_description: "Admin" }
    ))
}

#[get("/style.css")]
async fn style() -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/style.css")).await.ok()
}

#[get("/logo.png")]
async fn logo() -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/logo.png")).await.ok()
}

#[get("/script.js")]
async fn script() -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/script.js")).await.ok()
}

pub fn rocket() -> Rocket<Build> {
    rocket::build().attach(Template::fairing()).mount("/", routes![index, style, logo, script])
}