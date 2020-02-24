#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use std::io::Result;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> Result<NamedFile> {
    NamedFile::open("www/index.html")
}

#[catch(404)]
fn error_not_found() -> Result<NamedFile> {
    NamedFile::open("www/404.html")
}

#[get("/www/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("site/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files])
        .register(catchers![error_not_found])
        .launch();
}
