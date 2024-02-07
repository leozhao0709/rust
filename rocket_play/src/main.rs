use std::process;

#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello World!"
}

#[rocket::main]
async fn main() {
    println!("----PID---{}", process::id());
    let _ = rocket::build().mount("/", routes![hello]).launch().await;
}
