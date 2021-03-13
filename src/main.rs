extern crate sdl2;

mod cso;
mod point;
mod random;

use cso::CSO;
use point::Point;
use random::Random;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    let mut rnd = Random { seed: 5 };

    for _i in 0..10 {
        print!("Random float: {}\n", rnd.next_float());
        print!("Random bool: {}\n", rnd.next_bool());
    }

    const PX_SIZE: u32 = 8;
    let mut sim = CSO::new(32, 32);
    let drip_pt = Point::at(sim.width / 2, 0);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("cso", sim.width * PX_SIZE, sim.height * PX_SIZE)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        if i % 8 == 0 && sim.is_empty_at(&drip_pt) {
            sim.set(&drip_pt, 1);
        }
        sim.tick();

        i = (i + 1) % 255;
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for y in 0..sim.height {
            for x in 0..sim.width {
                if sim.get(&Point::at(x, y)) == 0 {
                    canvas.set_draw_color(Color::BLACK);
                } else {
                    canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
                }
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
