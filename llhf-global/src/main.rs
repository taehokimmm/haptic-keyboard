//! LLHF-Global: Low Latency Haptic Feedback with Global Input
//!
//! This is an iteration of the LLHF project that uses global input to trigger haptic feedback.
//! latency is not tested.

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use rdev::{Event, EventType, Key, listen};

use lazy_static::lazy_static;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use rodio::source::Buffered;

mod audio;

lazy_static! {
    static ref WAV : HashMap<char, Buffered<Decoder<BufReader<File>>>> = audio::load_sources();
    static ref OUTPUTSTREAMHANDLE: Arc<Mutex<Option<OutputStreamHandle>>> = Arc::new(Mutex::new(None));
}

pub fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut handle = OUTPUTSTREAMHANDLE.lock().unwrap();
    *handle = Some(stream_handle);
    drop(handle);

    if let Err(e) = listen(callback) {
        eprintln!("Error: {:?}", e);
    }
}

fn callback(event: Event) {
    if let EventType::KeyPress(key) = event.event_type {
        match key {
            Key::KeyA => play_sound('A'),
            Key::KeyB => play_sound('B'),
            Key::KeyC => play_sound('C'),
            Key::KeyD => play_sound('D'),
            Key::KeyE => play_sound('E'),
            Key::KeyF => play_sound('F'),
            Key::KeyG => play_sound('G'),
            Key::KeyH => play_sound('H'),
            Key::KeyI => play_sound('I'),
            Key::KeyJ => play_sound('J'),
            Key::KeyK => play_sound('K'),
            Key::KeyL => play_sound('L'),
            Key::KeyM => play_sound('M'),
            Key::KeyN => play_sound('N'),
            Key::KeyO => play_sound('O'),
            Key::KeyP => play_sound('P'),
            Key::KeyQ => play_sound('Q'),
            Key::KeyR => play_sound('R'),
            Key::KeyS => play_sound('S'),
            Key::KeyT => play_sound('T'),
            Key::KeyU => play_sound('U'),
            Key::KeyV => play_sound('V'),
            Key::KeyW => play_sound('W'),
            Key::KeyX => play_sound('X'),
            Key::KeyY => play_sound('Y'),
            Key::KeyZ => play_sound('Z'),
            _ => {}
        };
    }
}

fn play_sound(key: char) {
    let stream_handle = OUTPUTSTREAMHANDLE.lock().unwrap();
    match stream_handle.as_ref() {
        Some(handle) => {
            let source = WAV.get(&key).unwrap();
            handle.play_raw(source.clone().convert_samples()).unwrap();
            println!("Playing sound for {}", key)
        }
        None => {
            eprintln!("Error: No output stream handle found for {}", key);
        }
    }
}