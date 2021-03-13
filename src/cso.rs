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

    pub fn move_from_to(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        let value = self.get(x1, y1);
        self.set(x2, y2, value);
        self.set(x1, y1, 0);
    }

    fn tick_xy(&mut self, x: u32, y: u32) {
        if self.is_occupied_at(x, y) { return };

        if y > 0 {
            if self.is_occupied_at(x, y - 1) {
                self.move_from_to(x, y - 1, x, y);
            }
        }
    }

    pub fn tick(&mut self) {
        for y in (0..self.height).rev() {
            for x in (0..self.width).rev() {
                self.tick_xy(x, y);
            }
        }
    }
}
