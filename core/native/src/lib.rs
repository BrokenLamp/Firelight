#[allow(unused_imports)]
#[macro_use]
extern crate neon;
extern crate rodio;

pub mod game_state;
pub mod item;
pub mod plot;
pub mod plot_grid;
pub mod structure;
pub mod structures;

use neon::prelude::*;
use std::{io::BufReader, thread};

fn set_soundscape(mut cx: FunctionContext) -> JsResult<JsNumber> {
    thread::spawn(move || -> Option<()> {
        let device = rodio::default_output_device()?;
        let file =
            std::fs::File::open("resources/app.asar.unpacked/music/a-moment-of-sorrow.p.mp3")
                .ok()?;
        let sound = rodio::play_once(&device, BufReader::new(file)).ok()?;
        sound.set_volume(0.8);
        sound.sleep_until_end();
        println!("Playing sound");
        Some(())
    });
    Ok(cx.number(0.))
}

fn hello(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(178.5))
}

register_module!(mut cx, {
    cx.export_function("setSoundscape", set_soundscape)?;
    cx.export_function("hello", hello)?;
    Ok(())
});
