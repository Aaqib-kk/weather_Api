#[macro_use] extern crate rocket;
extern crate weather;

fn main() {
    rocket::ignite().mount("/", routes![weather::hello]).launch();
}