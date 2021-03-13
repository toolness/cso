use super::point::Point;
use super::random::Random;

pub struct CSO {
    arr: Vec<u8>,
    rng: Random,
    pub width: u32,
    pub height: u32,
}

impl CSO {
    pub fn new(width: u32, height: u32, rng: Random) -> CSO {
        CSO { arr: vec![0; (width * height) as usize], width, height, rng }
    }

    pub fn set(&mut self, point: &Point, value: u8) {
        self.arr[(point.y * self.width + point.x) as usize] = value;
    }

    pub fn get(&self, point: &Point) -> u8 {
        self.arr[(point.y * self.width + point.x) as usize]
    }

    pub fn is_occupied_at(&self, point: &Point) -> bool {
        self.get(point) != 0
    }

    pub fn is_empty_at(&self, point: &Point) -> bool {
        self.get(point) == 0
    }

    fn move_from_to(&mut self, from: &Point, to: &Point) {
        self.set(to, self.get(from));
        self.set(from, 0);
    }

    fn tick_point(&mut self, p: &Point) {
        if self.is_occupied_at(p) { return };

        if let Some(ref above) = p.above() {
            if self.is_occupied_at(above) {
                return self.move_from_to(above, p);
            }
            if let Some(ref above_left) = above.left() {
                if let Some(ref left) = p.left() {
                    if self.is_occupied_at(above_left) && self.is_occupied_at(left) {
                        let mut should_move = true;
                        if let Some(ref left_of_left) = left.left() {
                            if self.is_empty_at(left_of_left) {
                                // The grain is basically at the top of a mound and we
                                // should flip a coin to determine which direction it goes.
                                should_move = self.rng.next_bool();
                            }
                        }
                        if should_move {
                            return self.move_from_to(above_left, p);
                        }
                    }
                }
            }
            if let Some(ref above_right) = above.right_in(self) {
                if self.is_occupied_at(above_right) {
                    return self.move_from_to(above_right, p);
                }
            }
        }
    }

    pub fn tick(&mut self) {
        for y in (0..self.height).rev() {
            for x in (0..self.width).rev() {
                self.tick_point(&Point::at(x, y));
            }
        }
    }
}
