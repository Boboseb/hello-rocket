#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use std::path::*;
use rocket::response::NamedFile;
use rocket_contrib::Template;

#[get("/")]
fn index() -> &'static str {
    "Bonjour tout le monde !"
}

#[get("/echo/<str>")]
fn echo(str: &str) -> &str {
    str
}

#[get("/<a>/+/<b>")]
fn add(a: isize, b: isize) -> String {
    (a + b).to_string()
}

#[get("/echo2/<str>")]
fn echo2(str: &str) -> Template {
    Template::render("echo", &str)
}

#[get("/<file..>", rank = 1000)]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, echo, add, echo2, file]).launch();
}
