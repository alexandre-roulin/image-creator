use std::collections::HashMap;

use convert_case::{Case, Casing};
use csv::Reader;
use image::DynamicImage;

lazy_static! {
    pub static ref HEROES: HashMap<String, DynamicImage> = {
        let mut hashmap = HashMap::new();
        let mut rdr = Reader::from_path("heroes.csv").unwrap();
        for record in rdr.records() {
            let result = record.unwrap();
            let heroes = result.get(0).unwrap().to_case(Case::Snake);
            let file = format!("./icons/{}.png", heroes);

            hashmap.insert(heroes.to_owned(), image::open(file).unwrap());
        }
        hashmap
    };
}
