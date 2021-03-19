use super::cso::{CSO, Cell};
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
    pub sim: CSO,
}

pub const BMP_STATIC_COLOR: bmp::Pixel = bmp::consts::WHITE;
pub const BMP_SEWAGE_FACTORY_COLOR: bmp::Pixel = bmp::Pixel { r: 143, g: 86, b: 59 };
pub const BMP_WATER_FACTORY_COLOR: bmp::Pixel = bmp::Pixel { r: 95, g: 205, b: 228 };
pub const BMP_DRAIN_COLOR: bmp::Pixel = bmp::Pixel { r: 153, g: 229, b: 80 };
pub const BMP_WATER_COLOR: bmp::Pixel = bmp::Pixel { r: 91, g: 110, b: 225 };

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
    
        Level { sim, factories, drains, enable_water_factories: false, frame_number: 0 }
    }

    pub fn tick(&mut self) {
        self.frame_number = (self.frame_number + 1) % 255;
        let i = self.frame_number;
        for factory in self.factories.iter() {
            if factory.cell == Cell::Water && !self.enable_water_factories {
                continue;
            }
            if i % factory.interval < factory.count && self.sim.is_empty_at(&factory.point) {
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
