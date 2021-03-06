use super::point::Point;
use super::random::Random;

pub const SEWAGE_PURITY: u8 = 0;
pub const MAX_PURITY: u8 = 100;
const SEWAGE_SPREAD_RATE: u8 = 5;
const MAX_PURITY_TO_SPREAD_SEWAGE: u8 = 25;
const MIN_PURITY_TO_SLIDE: u8 = 75;

#[derive(PartialEq, Clone, Copy)]
pub enum Cell {
    Empty,
    Static,
    Sand,
    Water(u8),
}

pub struct CSO {
    arr: Vec<Cell>,
    rng: Random,
    pub width: u32,
    pub height: u32,
}

struct Displacement {
    start: Point,
    end: Point,
    direction: i8,
}

impl CSO {
    pub fn new(width: u32, height: u32, rng: Random) -> CSO {
        CSO { arr: vec![Cell::Empty; (width * height) as usize], width, height, rng }
    }

    pub fn set(&mut self, point: &Point, value: Cell) {
        self.arr[(point.y * self.width + point.x) as usize] = value;
    }

    pub fn get(&self, point: &Point) -> Cell {
        self.arr[(point.y * self.width + point.x) as usize]
    }

    pub fn is_occupied_at(&self, point: &Point) -> bool {
        self.get(point) != Cell::Empty
    }

    pub fn is_movable_at(&self, point: &Point) -> bool {
        match self.get(point) {
            Cell::Empty | Cell::Static => false,
            _ => true,
        }
    }

    pub fn is_empty_at(&self, point: &Point) -> bool {
        self.get(point) == Cell::Empty
    }

    pub fn is_liquid_at(&self, point: &Point) -> bool {
        match self.get(point) {
            Cell::Water(..) => true,
            _ => false
        }
    }

    fn move_from_to(&mut self, from: &Point, to: &Point) {
        self.set(to, self.get(from));
        self.set(from, Cell::Empty);
    }

    fn in_lateral_direction(&self, p: &Point, direction: i8) -> Option<Point> {
        if direction > 0 { p.right_in(self) } else { p.left() }
    }

    fn get_closest_path_down(&mut self, start_point: &Point) -> Option<Point> {
        let mut to_explore: Vec<(Point, i8)> = vec![(*start_point, -1), (*start_point, 1)];
        let mut best: Option<(i8, i32)> = None;

        while let Some((p, direction)) = to_explore.pop() {
            let maybe_neighbor = self.in_lateral_direction(&p, direction);
            if let Some(neighbor) = maybe_neighbor {
                if self.is_empty_at(&neighbor) {
                    if let Some(below_neighbor) = neighbor.below_in(self) {
                        if self.is_empty_at(&below_neighbor) {
                            let dist = (neighbor.x as i32 - start_point.x as i32).abs();
                            let mut set_best = false;
                            if let Some((_, best_dist)) = best {
                                if dist < best_dist {
                                    set_best = true;
                                } else if dist == best_dist {
                                    set_best = self.rng.next_bool();
                                }
                            } else {
                                set_best = true;
                            }
                            if set_best {
                                best = Some((direction, dist));
                            }
                        } else {
                            to_explore.push((neighbor, direction));
                        }
                    }
                }
            }
        }

        match best {
            Some((direction, _)) => self.in_lateral_direction(start_point, direction),
            None => None,
        }
    }

    fn get_liquid_displacement(&self, start_point: Point) -> Option<Displacement> {
        let mut to_explore: Vec<(Point, i8)> = vec![(start_point, -1), (start_point, 1)];
        let mut best: Option<(Point, i32, i8)> = None;

        while let Some((p, direction)) = to_explore.pop() {
            let maybe_neighbor = self.in_lateral_direction(&p, direction);
            if let Some(neighbor) = maybe_neighbor {
                if self.is_empty_at(&neighbor) {
                    let dist = ((neighbor.x as i32 - start_point.x as i32) + (neighbor.y as i32 - start_point.y as i32)).abs();
                    if let Some((_, best_dist, _)) = best {
                        if dist < best_dist {
                            best = Some((neighbor, dist, direction));
                        }
                    } else {
                        best = Some((neighbor, dist, direction));
                    }
                } else if self.is_liquid_at(&neighbor) {
                    to_explore.push((neighbor, direction));
                }
            }
        }

        match best {
            Some((end, _, direction)) => Some(Displacement { start: start_point, end, direction }),
            None => None,
        }
    }

    fn apply_displacement(&mut self, dis: &Displacement) {
        assert_eq!(dis.start.y, dis.end.y);
        let mut point = dis.end;
        while point.x != dis.start.x {
            let from_point = self.in_lateral_direction(&point, -dis.direction).unwrap();
            self.move_from_to(&from_point, &point);
            point = from_point;
        }
    }

    fn maybe_spread_sewage(&mut self, point: &Point, maybe_other: &Option<Point>) {
        if let Some(other) = maybe_other {
            let point_cell = self.get(point);
            let other_cell = self.get(other);
            if let (Cell::Water(mut point_purity), Cell::Water(mut other_purity)) = (point_cell, other_cell) {
                if point_purity < MAX_PURITY_TO_SPREAD_SEWAGE {
                    other_purity = other_purity.checked_sub(SEWAGE_SPREAD_RATE).unwrap_or(0);
                }
                if other_purity < MAX_PURITY_TO_SPREAD_SEWAGE {
                    point_purity = point_purity.checked_sub(SEWAGE_SPREAD_RATE).unwrap_or(0);
                }
                self.set(point, Cell::Water(point_purity));
                self.set(other, Cell::Water(other_purity));
            }
        }
    }

    fn tick_point(&mut self, p: &Point) {
        if self.is_liquid_at(p) {
            if let Some(ref above) = p.above() {
                if let Cell::Water(purity) = self.get(above) {
                    if purity >= MIN_PURITY_TO_SLIDE {
                        if let Some(ref to) = self.get_closest_path_down(above) {
                            return self.move_from_to(above, to);
                        }
                    }
                    if let Some(ref displacement) = self.get_liquid_displacement(*p) {
                        self.apply_displacement(&displacement);
                        return self.move_from_to(above, p);
                    }
                }
            }
            self.maybe_spread_sewage(p, &p.left());
            self.maybe_spread_sewage(p, &p.above());
        }

        if self.is_occupied_at(p) { return };

        if let Some(ref above) = p.above() {
            if self.is_movable_at(above) {
                return self.move_from_to(above, p);
            }
            if let Some(ref above_left) = above.left() {
                if let Some(ref left) = p.left() {
                    if self.is_movable_at(above_left) && self.is_occupied_at(left) {
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
                if self.is_movable_at(above_right) {
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
