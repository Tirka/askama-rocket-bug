#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[get("/")]
fn hello() -> HelloTemplate<'static> {
    HelloTemplate { name: "world" }
}

fn main() {
    let rocket = rocket::ignite().mount("/", routes![hello]).launch();
}