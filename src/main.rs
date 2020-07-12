#[macro_use] extern crate rocket;
extern crate weather_RUST_API;

fn main() {
    rocket::ignite().mount("/", routes![weather_RUST_API::hello]).launch();
}