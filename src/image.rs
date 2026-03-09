#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<u8>
}

impl Image {
    pub fn new(width: usize, height: usize)->Self{
        Self {width, height, data: vec![0; width*height]}
    }
    pub(crate) fn set(&mut self, x: i32, y: i32, color: u8) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }

        let idx = y as usize * self.width + x as usize;
        self.data[idx] = color as u8;
    }
    pub fn get(&self, x: i32, y: i32) -> u8 {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return 0;
        }

        let idx = y as usize * self.width + x as usize;
        return self.data[idx];
    }

}