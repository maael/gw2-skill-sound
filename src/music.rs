use crate::logging;
use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::*;
// use rodio::{source::Source, Decoder, OutputStream};
use rodio::source::FadeIn;
use rodio::source::Source;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::RwLock;
use std::{thread, time};

fn list_devices() {
    let host = cpal::default_host();
    let devices = host.output_devices().unwrap();
    for device in devices {
        let dev: rodio::Device = device.into();
        let dev_name: String = dev.name().unwrap();
        logging::info(format!(" # Device : {}", dev_name));
    }
}

fn get_output_stream(device_name: &str) -> (OutputStream, OutputStreamHandle) {
    let host = cpal::default_host();
    let devices = host.output_devices().unwrap();
    let (mut _stream, mut stream_handle) = OutputStream::try_default().unwrap();
    for device in devices {
        let dev: rodio::Device = device.into();
        let dev_name: String = dev.name().unwrap();
        if dev_name == device_name {
            logging::info(format!("Device found: {}", dev_name));
            (_stream, stream_handle) = OutputStream::try_from_device(&dev).unwrap();
        }
    }
    return (_stream, stream_handle);
}

static MAX_VOL: f32 = 0.1;
static mut PLAYING: RwLock<f32> = RwLock::new(0.0);

pub fn is_playing() -> bool {
    let current = unsafe { *PLAYING.read().unwrap() };
    current > 0.0
}

pub fn is_stopping() -> bool {
    let current = unsafe { *PLAYING.read().unwrap() };
    current < MAX_VOL
}

fn file_source() -> FadeIn<Decoder<BufReader<File>>> {
    let file = BufReader::new(File::open("shroud.ogg").unwrap());
    Decoder::new(file)
        .unwrap()
        .fade_in(std::time::Duration::from_secs(1))
}

pub fn play_music() {
    if is_playing() {
        logging::info(String::from("Skipping start, already playing"));
        return;
    }
    logging::info(String::from("Starting sound"));
    unsafe {
        {
            let mut w = PLAYING.write().unwrap();
            *w = 0.0;
        }
    }
    list_devices();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = file_source();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    logging::info(String::from("[play_music] Start"));
    sink.set_volume(MAX_VOL);

    unsafe {
        {
            let mut w = PLAYING.write().unwrap();
            *w = MAX_VOL;
        }
        {
            while *PLAYING.read().unwrap() > 0.0 && !sink.empty() {
                let vol = *PLAYING.read().unwrap();
                sink.set_volume(vol);
                thread::sleep(time::Duration::from_millis(100))
            }
        }
    }

    logging::info(String::from("[play_music] End"));
}

pub fn stop_music() {
    if is_stopping() {
        logging::info(String::from("Skipping stop, already playing"));
        return;
    }
    logging::info(String::from("Stopping"));
    unsafe {
        while *PLAYING.read().unwrap() > 0.0 {
            let new_vol = {
                let current = *PLAYING.read().unwrap();
                current - 0.001
            };
            logging::info(format!("Set Vol: {}", new_vol));
            let mut w = PLAYING.write().unwrap();
            *w = new_vol;
            thread::sleep(time::Duration::from_millis(50))
        }
    }
    logging::info(String::from("Stopped"));
}
