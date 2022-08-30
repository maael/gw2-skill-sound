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

    match file.write_all("[start] Starting".as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

pub fn info() {
    let path = Path::new("gw2_skill_sound.log");
    let display = path.display();
    let mut file = match OpenOptions::new().append(true).open(path) {
        Err(why) => panic!("couldn't append {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all("[info] Some information".as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
