extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use csolib::cso::{CSO, Cell};
use csolib::point::Point;
use csolib::random::Random;

const PX_SIZE: u32 = 8;
const INITIAL_FPS: u32 = 15;
const FPS_INCREMENT: u32 = 15;
const MAX_FPS: u32 = 60;

const BMP_STATIC_COLOR: bmp::Pixel = bmp::consts::WHITE;
const BMP_SEWAGE_FACTORY_COLOR: bmp::Pixel = bmp::Pixel { r: 143, g: 86, b: 59 };
const BMP_WATER_FACTORY_COLOR: bmp::Pixel = bmp::Pixel { r: 95, g: 205, b: 228 };
const BMP_DRAIN_COLOR: bmp::Pixel = bmp::Pixel { r: 153, g: 229, b: 80 };
const BMP_WATER_COLOR: bmp::Pixel = bmp::Pixel { r: 91, g: 110, b: 225 };

const STATIC_COLOR: Color = Color::WHITE;
const EMPTY_COLOR: Color = Color::BLACK;
const SEWAGE_COLOR: Color = bmp_to_sdl_color(&BMP_SEWAGE_FACTORY_COLOR);
const WATER_COLOR: Color = bmp_to_sdl_color(&BMP_WATER_COLOR);

const fn bmp_to_sdl_color(color: &bmp::Pixel) -> Color {
    Color::RGBA(color.r, color.g, color.b, 255)
}

pub struct CellFactory {
    pub point: Point,
    pub cell: Cell,
    pub interval: u8,
    pub count: u8
}

pub struct CellDrain {
    pub point: Point,
    pub interval: u8,
}

fn main() {
    let level = bmp::open("level.bmp").unwrap();
    let mut sim = CSO::new(level.get_width(), level.get_height(), Random { seed: 5 });
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut factories: Vec<CellFactory> = vec![];
    let mut drains: Vec<CellDrain> = vec![];
    let mut enable_water_factories: bool = false;
    let mut fps = INITIAL_FPS;

    print!("Combined Sewage Overflow simulator\n");
    print!("Keys:\n");
    print!("R - Toggle rain\n");
    print!("- - Decrease FPS\n");
    print!("= - Increase FPS\n");

    let window = video_subsystem.window("cso", sim.width * PX_SIZE, sim.height * PX_SIZE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    for (x, y) in level.coordinates() {
        let value = level.get_pixel(x, y);
        let point = Point::at(x, y);
        match value {
            BMP_STATIC_COLOR => {
                sim.set(&point, Cell::Static);
            }
            BMP_WATER_FACTORY_COLOR => {
                factories.push(CellFactory {
                    point,
                    cell: Cell::Water,
                    interval: 8,
                    count: 2
                });
            }
            BMP_SEWAGE_FACTORY_COLOR => {
                factories.push(CellFactory {
                    point,
                    cell: Cell::Sewage,
                    interval: 8,
                    count: 1
                });
            }
            BMP_DRAIN_COLOR => {
                drains.push(CellDrain {
                    point,
                    interval: 16
                });
            }
            BMP_WATER_COLOR => {
                sim.set(&point, Cell::Water);
            }
            _ => {}
        }
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        for factory in factories.iter() {
            if factory.cell == Cell::Water && !enable_water_factories {
                continue;
            }
            if i % factory.interval < factory.count && sim.is_empty_at(&factory.point) {
                sim.set(&factory.point, factory.cell);
            }
        }
        for drain in drains.iter() {
            if i % drain.interval == 0 {
                sim.set(&drain.point, Cell::Empty);
            }
        }
        sim.tick();

        i = (i + 1) % 255;
        for y in 0..sim.height {
            for x in 0..sim.width {
                let color: Color = match sim.get(&Point::at(x, y)) {
                    Cell::Empty => { EMPTY_COLOR }
                    Cell::Static => { STATIC_COLOR }
                    Cell::Sand => { SEWAGE_COLOR }
                    Cell::Water => { WATER_COLOR }
                    Cell::Sewage => { SEWAGE_COLOR }
                };
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
                    enable_water_factories = !enable_water_factories;
                    print!("Rain {}.\n", if enable_water_factories { "enabled" } else { "disabled" });
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
