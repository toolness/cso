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

const PX_SIZE: u32 = 8;
const FPS: u32 = 15;

const BMP_STATIC_COLOR: bmp::Pixel = bmp::consts::WHITE;
const BMP_SEWAGE_FACTORY_COLOR: bmp::Pixel = bmp::Pixel { r: 143, g: 86, b: 59 };
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

fn main() {
    let env = bmp::open("environment.bmp").unwrap();
    let mut sim = CSO::new(env.get_width(), env.get_height(), Random { seed: 5 });
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut factories: Vec<CellFactory> = vec![
        CellFactory { point: Point::at(sim.width / 2, 0), cell: Cell::Water, interval: 8, count: 2 },
    ];

    let window = video_subsystem.window("cso", sim.width * PX_SIZE, sim.height * PX_SIZE)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();

    for (x, y) in env.coordinates() {
        let value = env.get_pixel(x, y);
        match value {
            BMP_STATIC_COLOR => {
                sim.set(&Point::at(x, y), Cell::Static);
            }
            BMP_SEWAGE_FACTORY_COLOR => {
                factories.push(CellFactory {
                    point: Point::at(x, y),
                    cell: Cell::Sewage,
                    interval: 8,
                    count: 1
                });
            }
            BMP_WATER_COLOR => {
                sim.set(&Point::at(x, y), Cell::Water);
            }
            _ => {}
        }
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        for factory in factories.iter() {
            if i % factory.interval < factory.count && sim.is_empty_at(&factory.point) {
                sim.set(&factory.point, factory.cell);
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
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
