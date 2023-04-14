use qoi_rs::decode;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::{env, process};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Give a path to a .qoi image");
        process::exit(-1);
    } else {
        let file = args.nth(1).unwrap();
        let points = match decode(file.into()) {
            Ok(points) => points,
            Err(e) => {
                eprintln!("Error: {e}");
                process::exit(-1);
            }
        };
        for point in points {
            canvas.set_draw_color(point.pixel);
            match canvas.draw_point((point.x as i32, point.y as i32)) {
                Ok(()) => (),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        canvas.present();
    }
    'running: loop {
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }
}
