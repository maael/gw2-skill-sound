use crate::{
    logging,
    mumblelink::get_link,
    music::{is_playing, play_music, stop_music},
};
use inputbot::KeybdKey::*;
use std::thread;

pub fn setup() {
    thread::spawn(move || {
        logging::info(String::from("[events] setup"));
        F1Key.bind(|| {
            let id = get_link();
            /*
             * Necro profession = 8
             * Necro = 53
             * Reaper = 34
             */
            if id.profession != 8 || id.spec != 34 && id.spec != 53 {
                return;
            }
            if is_playing() {
                stop_music()
            } else {
                play_music()
            }
        });

        inputbot::handle_input_events();
        logging::info(String::from("[events] handling"));
    });
}
