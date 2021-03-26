use wasm_bindgen::prelude::*;
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

    pub fn set_enable_water_factories(&mut self, enable: bool) {
        self.level.enable_water_factories = enable;
    }

    pub fn set_override_water_factory_count(&mut self, count: Option<u8>) {
        self.level.override_water_factory_count = count;
    }

    pub fn draw(&self, arr: &mut [u8]) {
        let mut i = 0;
        for y in 0..self.level.sim.height {
            for x in 0..self.level.sim.width {
                let color = self.level.get_color(&Point::at(x, y));
                arr[i] = color.r;
                arr[i + 1] = color.g;
                arr[i + 2] = color.b;
                arr[i + 3] = 255;
                i += 4;
            }
        }
    }

    pub fn tick(&mut self) {
        self.level.tick();
    }
}
