use std::fs::File;
use std::io::{Write, BufWriter};

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

    pub fn save_as_pgm(&self, path: &str)->std::io::Result<()> {
        assert_eq!(self.data.len(), self.width * self.height);

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P2")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;

        for row in self.data.chunks(self.width) {
            for (i, &pixel) in row.iter().enumerate() {
                if i > 0 {
                    write!(writer, " ")?;
                }
                write!(writer, "{pixel}")?;
            }
            writeln!(writer)?;
        }

        writer.flush()?;
        Ok(())
    }
}