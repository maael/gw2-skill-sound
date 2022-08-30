use crate::logging;
use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::*;
// use rodio::{source::Source, Decoder, OutputStream};
// use std::fs::File;
// use std::io::BufReader;
use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::time::Duration;

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

pub fn play_music() {
    logging::info(String::from("Starting sound"));
    list_devices();
    let (_stream, stream_handle) = get_output_stream("Headphones (6- Arctis Pro Wireless Game)");
    // logging::info(String::from("Got stream"));
    // let file = BufReader::new(File::open("shroud.ogg").unwrap());
    // logging::info(String::from("Got file shroud.ogg"));
    // // Decode that sound file into a source
    // let source = Decoder::new(file).unwrap();
    // logging::info(String::from("Got source"));
    // // Play the sound directly on the device
    // stream_handle.play_raw(source.convert_samples()).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(0.25))
        .amplify(0.20);
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
    logging::info(String::from("Playing"));
}
