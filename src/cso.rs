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
}
