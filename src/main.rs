extern crate sdl2;

mod cso;
mod point;
mod random;

use cso::{CSO, Cell};
use point::Point;
use random::Random;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    const PX_SIZE: u32 = 8;
    const FRAMES_PER_DRIP: u8 = 8;
    let env = bmp::open("environment.bmp").unwrap();
    let mut sim = CSO::new(env.get_width(), env.get_height(), Random { seed: 5 });
    let drip_pt = Point::at(sim.width / 2, 0);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("cso", sim.width * PX_SIZE, sim.height * PX_SIZE)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();

    for (x, y) in env.coordinates() {
        let value = env.get_pixel(x, y);
        if value == bmp::consts::WHITE {
            sim.set(&Point::at(x, y), Cell::Static);
        }
    }

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        if i % FRAMES_PER_DRIP == 0 && sim.is_empty_at(&drip_pt) {
            sim.set(&drip_pt, Cell::Water);
        }
        sim.tick();

        i = (i + 1) % 255;
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for y in 0..sim.height {
            for x in 0..sim.width {
                let color: Color = match sim.get(&Point::at(x, y)) {
                    Cell::Empty => {
                        Color::BLACK
                    }
                    Cell::Static => {
                        Color::WHITE
                    },
                    Cell::Sand => {
                        Color::RGB(i, 64, 255 - i)
                    },
                    Cell::Water => {
                        Color::RGB(0, 64, 180 + (i % 70))
                    },
                };
                canvas.set_draw_color(color);
                canvas.fill_rect(Rect::new((x * PX_SIZE) as i32, (y * PX_SIZE) as i32, PX_SIZE, PX_SIZE)).unwrap();
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
