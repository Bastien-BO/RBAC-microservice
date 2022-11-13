#[macro_use] extern crate rocket;

use rocket::response::content::Json;

fn main() {
    rocket::ignite()
        .mount("/permission", routes![hello])
        .launch();
}