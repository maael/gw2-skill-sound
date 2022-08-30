static LOREM_IPSUM: &str = "[start] Starting";

use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn setup() {
    let path = Path::new("gw2_skill_sound.log");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

pub fn info(str: String) {
    let path = Path::new("gw2_skill_sound.log");
    let display = path.display();
    let mut file = match OpenOptions::new().append(true).open(path) {
        Err(why) => panic!("couldn't append {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(format!("\n[info] {}", str).as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
