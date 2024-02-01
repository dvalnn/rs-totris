#![allow(dead_code, unused_variables)]

use cgmath::Vector2;
use sdl2::{event::Event, pixels::Color};

use crate::engine::Engine;

const INIT_SIZE: Vector2<u32> = Vector2::new(1024, 1024);
const BACKGROUND_COLOR: Color = Color::RGB(0x10, 0x10, 0x18);

pub fn run(_engine: Engine) {
    let sdl = sdl2::init().expect("SDL2 initialization failed");

    let mut canvas = {
        let video = sdl
            .video()
            .expect("SDL2 video subsystem initialization failed");

        let window = video
            .window("rs-totris", 640, 480)
            .position_centered()
            .resizable()
            .build()
            .expect("Window creation failed");

        window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .expect("Canvas creation failed")
    };

    let mut events = sdl.event_pump().expect("Event pump creation failed");

    loop {
        for event in events.poll_iter() {
            #[allow(clippy::single_match)]
            match dbg!(event) {
                Event::Quit { .. } => return,
                _ => {}
            }
        }
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();

        //NOTE: draw graphics here

        canvas.present();
    }
}
