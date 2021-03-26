use super::cso::{CSO, Cell, MAX_PURITY, SEWAGE_PURITY};
use super::random::Random;
use super::point::Point;

struct CellFactory {
    pub point: Point,
    pub cell: Cell,
    pub interval: u8,
    pub count: u8
}

struct CellDrain {
    pub point: Point,
    pub interval: u8,
}

pub struct Level {
    factories: Vec<CellFactory>,
    drains: Vec<CellDrain>,
    frame_number: u8,
    pub enable_water_factories: bool,
    pub override_water_factory_count: Option<u8>,
    pub sim: CSO,
}

pub const BMP_STATIC_COLOR: bmp::Pixel = bmp::consts::WHITE;
pub const BMP_EMPTY_COLOR: bmp::Pixel = bmp::consts::BLACK;
pub const BMP_SEWAGE_FACTORY_COLOR: bmp::Pixel = bmp::Pixel { r: 143, g: 86, b: 59 };
pub const BMP_WATER_FACTORY_COLOR: bmp::Pixel = bmp::Pixel { r: 95, g: 205, b: 228 };
pub const BMP_DRAIN_COLOR: bmp::Pixel = bmp::Pixel { r: 153, g: 229, b: 80 };
pub const BMP_WATER_COLOR: bmp::Pixel = bmp::Pixel { r: 91, g: 110, b: 225 };

fn clamped_u8(value: i32) -> u8 {
    if value < u8::MIN as i32 {
        u8::MIN
    } else if value > u8::MAX as i32 {
        u8::MAX
    } else {
        value as u8
    }
}

fn lerp_u8(a: u8, b: u8, amount: f32) -> u8 {
    let max_delta: i32 = b as i32 - a as i32;
    let value = (a as f32 + max_delta as f32 * amount) as i32;
    clamped_u8(value)
}

fn lerp_color(a: bmp::Pixel, b: bmp::Pixel, amount: f32) -> bmp::Pixel {
    bmp::Pixel {
        r: lerp_u8(a.r, b.r, amount),
        g: lerp_u8(a.g, b.g, amount),
        b: lerp_u8(a.b, b.b, amount),
    }
}

impl Level {
    pub fn from_bmp(image: bmp::Image) -> Level {
        let mut sim = CSO::new(image.get_width(), image.get_height(), Random { seed: 5 });
        let mut factories: Vec<CellFactory> = vec![];
        let mut drains: Vec<CellDrain> = vec![];

        for (x, y) in image.coordinates() {
            let value = image.get_pixel(x, y);
            let point = Point::at(x, y);
            match value {
                BMP_STATIC_COLOR => {
                    sim.set(&point, Cell::Static);
                }
                BMP_WATER_FACTORY_COLOR => {
                    factories.push(CellFactory {
                        point,
                        cell: Cell::Water(MAX_PURITY),
                        interval: 8,
                        count: 2
                    });
                }
                BMP_SEWAGE_FACTORY_COLOR => {
                    factories.push(CellFactory {
                        point,
                        cell: Cell::Water(SEWAGE_PURITY),
                        interval: 8,
                        count: 1
                    });
                }
                BMP_DRAIN_COLOR => {
                    drains.push(CellDrain {
                        point,
                        interval: 12
                    });
                }
                BMP_WATER_COLOR => {
                    sim.set(&point, Cell::Water(MAX_PURITY));
                }
                _ => {}
            }
        }
    
        Level { sim, factories, drains, enable_water_factories: false, frame_number: 0, override_water_factory_count: None }
    }

    pub fn get_color(&self, point: &Point) -> bmp::Pixel {
        match self.sim.get(&point) {
            Cell::Empty => { BMP_EMPTY_COLOR }
            Cell::Static => { BMP_STATIC_COLOR }
            Cell::Sand => { BMP_SEWAGE_FACTORY_COLOR }
            Cell::Water(purity) => {
                lerp_color(BMP_SEWAGE_FACTORY_COLOR, BMP_WATER_COLOR, purity as f32 / MAX_PURITY as f32)
            }
        }
    }

    pub fn tick(&mut self) {
        self.frame_number = (self.frame_number + 1) % 255;
        let i = self.frame_number;
        for factory in self.factories.iter() {
            let mut count = factory.count;
            if let Cell::Water(MAX_PURITY) = factory.cell {
                if !self.enable_water_factories {
                    continue;
                }
                count = self.override_water_factory_count.unwrap_or(count);
            }
            if i % factory.interval < count && self.sim.is_empty_at(&factory.point) {
                self.sim.set(&factory.point, factory.cell);
            }
        }
        for drain in self.drains.iter() {
            if i % drain.interval == 0 {
                self.sim.set(&drain.point, Cell::Empty);
            }
        }
        self.sim.tick();
    }
}
