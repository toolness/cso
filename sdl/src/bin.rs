extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use csolib::point::Point;
use csolib::level::Level;

const PX_SIZE: u32 = 8;
const INITIAL_FPS: u32 = 15;
const FPS_INCREMENT: u32 = 15;
const MAX_FPS: u32 = 60;

const fn bmp_to_sdl_color(color: &bmp::Pixel) -> Color {
    Color::RGBA(color.r, color.g, color.b, 255)
}

fn main() {
    let mut level = Level::from_bmp(bmp::open("level.bmp").unwrap());
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut fps = INITIAL_FPS;

    print!("Combined Sewage Overflow simulator\n");
    print!("Keys:\n");
    print!("R   Toggle rain\n");
    print!("-   Decrease FPS\n");
    print!("=   Increase FPS\n");
    print!("ESC Quit\n");

    let window = video_subsystem.window("cso", level.sim.width * PX_SIZE, level.sim.height * PX_SIZE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        level.tick();

        for y in 0..level.sim.height {
            for x in 0..level.sim.width {
                let color = bmp_to_sdl_color(&level.get_color(&Point::at(x, y)));
                canvas.set_draw_color(color);
                canvas.fill_rect(Rect::new((x * PX_SIZE) as i32, (y * PX_SIZE) as i32, PX_SIZE, PX_SIZE)).unwrap();
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    print!("Exiting.\n");
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    level.enable_water_factories = !level.enable_water_factories;
                    print!("Rain {}.\n", if level.enable_water_factories { "enabled" } else { "disabled" });
                },
                Event::KeyDown { keycode: Some(keycode), .. } if keycode == Keycode::Equals || keycode == Keycode::Minus => {
                    if keycode == Keycode::Equals {
                        fps += FPS_INCREMENT;
                    } else {
                        fps -= FPS_INCREMENT;
                    }
                    if fps <= 0 {
                        fps = FPS_INCREMENT;
                    } else if fps > MAX_FPS {
                        fps = MAX_FPS;
                    }
                    print!("FPS: {}.\n", fps);
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps));
    }
}
