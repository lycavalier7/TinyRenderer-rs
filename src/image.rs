use std::fs::{create_dir_all, File};
use std::io::{Write, BufWriter};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<Color>
}

impl Image {
    pub fn new(width: usize, height: usize)->Self{
        Self {
            width,
            height,
            data: vec![Color{r: 0, g: 0, b: 0, a: 255}; width*height]}
    }
    pub(crate) fn set(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }

        let idx = y as usize * self.width + x as usize;
        self.data[idx] = color as Color;
    }
    pub fn get(&self, x: i32, y: i32) -> Color {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return Color{r: 0, g: 0, b: 0, a: 255};
        }

        let idx = y as usize * self.width + x as usize;
        return self.data[idx].clone();
    }

    pub fn save_as_ppm(&self, path: &str)->std::io::Result<()> {
        assert_eq!(self.data.len(), self.width * self.height);

        let p = Path::new(path);

        if let Some(parent) = p.parent() {
            if parent.as_os_str().is_empty() == false {
                create_dir_all(parent)?;
            }
        }

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;

        for row in self.data.chunks(self.width) {
            for (i, &pixel) in row.iter().enumerate() {
                if i > 0 {
                    write!(writer, " ")?;
                }
                write!{writer, "{} {} {}", pixel.r, pixel.g, pixel.b}?;
            }
            writeln!(writer)?;
        }

        writer.flush()?;
        Ok(())
    }
}