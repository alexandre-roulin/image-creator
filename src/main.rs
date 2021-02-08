use config::Config;

#[macro_use]
extern crate lazy_static;

mod config;
mod constant;
mod image;
mod model;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        let config = Config::new(args[1].as_str()).unwrap();
        image::create_image_heroes(&config.players);
    } else { 
        eprintln!("Need to specify filename");
    }
}
