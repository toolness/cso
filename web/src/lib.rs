use wasm_bindgen::prelude::*;
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
}
