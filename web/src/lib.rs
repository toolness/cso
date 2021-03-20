use wasm_bindgen::prelude::*;
use csolib::cso::Cell;
use csolib::point::Point;
use csolib::level::Level;

#[wasm_bindgen]
pub struct WebLevel {
    level: Level,
}

#[wasm_bindgen]
impl WebLevel {
    pub fn new(bmp_bytes: &[u8]) -> WebLevel {
        let mut mutable_bytes = bmp_bytes;
        let bmp = bmp::from_reader(&mut mutable_bytes).unwrap();
        let level = Level::from_bmp(bmp);
        WebLevel { level }
    }

    pub fn get_width(&self) -> u32 {
        self.level.sim.width
    }

    pub fn get_height(&self) -> u32 {
        self.level.sim.height
    }

    pub fn draw(&self, arr: &mut [u8]) {
        let mut i = 0;
        for y in 0..self.level.sim.height {
            for x in 0..self.level.sim.width {
                let color: &bmp::Pixel = match self.level.sim.get(&Point::at(x, y)) {
                    Cell::Empty => { &bmp::consts::BLACK }
                    Cell::Static => { &bmp::consts::WHITE }
                    Cell::Sand => { &csolib::level::BMP_SEWAGE_FACTORY_COLOR }
                    Cell::Water => { &csolib::level::BMP_WATER_COLOR }
                    Cell::Sewage => { &csolib::level::BMP_SEWAGE_FACTORY_COLOR }
                };
                arr[i] = color.r;
                arr[i + 1] = color.g;
                arr[i + 2] = color.b;
                arr[i + 3] = 255;
                i += 4;
            }
        }
    }
}
