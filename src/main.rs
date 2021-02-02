use config::Config;

#[macro_use]
extern crate lazy_static;

mod config;
mod constant;
mod image;
mod model;

fn main() {
    let config = Config::new().unwrap();
    image::create_image_heroes(&config.players);
}
