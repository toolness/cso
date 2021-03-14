use super::cso::CSO;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn at(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    pub fn above(&self) -> Option<Point> {
        if self.y > 0 { Some(Point::at(self.x, self.y - 1)) } else { None }
    }

    pub fn left(&self) -> Option<Point> {
        if self.x > 0 { Some(Point::at(self.x - 1, self.y)) } else { None }
    }

    pub fn right_in(&self, cso: &CSO) -> Option<Point> {
        if self.x < cso.width - 1 { Some(Point::at(self.x + 1, self.y)) } else { None }
    }
}
