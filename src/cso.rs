struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    fn at(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    pub fn is_occupied_in(&self, cso: &CSO) -> bool {
        cso.is_occupied_at(self.x, self.y)
    }

    pub fn above(&self) -> Option<Point> {
        if self.y > 0 { Some(Point::at(self.x, self.y - 1)) } else { None }
    }

    pub fn left(&self) -> Option<Point> {
        if self.x > 0 { Some(Point::at(self.x - 1, self.y)) } else { None }
    }

    pub fn right_in(&self, cso: &CSO) -> Option<Point> {
        if self.x < cso.width { Some(Point::at(self.x + 1, self.y)) } else { None }
    }
}

pub struct CSO {
    arr: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl CSO {
    pub fn new(width: u32, height: u32) -> CSO {
        CSO { arr: vec![0; (width * height) as usize], width, height }
    }

    pub fn set(&mut self, x: u32, y: u32, value: u8) {
        self.arr[(y * self.width + x) as usize] = value;
    }

    pub fn get(&self, x: u32, y: u32) -> u8 {
        self.arr[(y * self.width + x) as usize]
    }

    pub fn is_occupied_at(&self, x: u32, y: u32) -> bool {
        self.get(x, y) != 0
    }

    pub fn is_empty_at(&self, x: u32, y: u32) -> bool {
        self.get(x, y) == 0
    }

    fn move_from_to(&mut self, from: Point, to: Point) {
        let value = self.get(from.x, from.y);
        self.set(to.x, to.y, value);
        self.set(from.x, from.y, 0);
    }

    fn tick_point(&mut self, p: Point) {
        if p.is_occupied_in(self) { return };

        if let Some(above) = p.above() {
            if above.is_occupied_in(self) {
                return self.move_from_to(above, p);
            }
            if let Some(above_left) = above.left() {
                if let Some(left) = p.left() {
                    if above_left.is_occupied_in(self) && left.is_occupied_in(self) {
                        return self.move_from_to(above_left, p);
                    }
                }
            }
            if let Some(above_right) = above.right_in(self) {
                if above_right.is_occupied_in(self) {
                    return self.move_from_to(above_right, p);
                }
            }
        }
    }

    pub fn tick(&mut self) {
        for y in (0..self.height).rev() {
            for x in (0..self.width).rev() {
                self.tick_point(Point::at(x, y));
            }
        }
    }
}
