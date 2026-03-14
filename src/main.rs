use crate::geometry::Vec2i;
use crate::image::Color;

mod geometry;
mod image;
mod raster;

fn main() {
    let width = 100;
    let height = 100;

    let mut image = image::Image::new(width, height);
    let p0 = Vec2i::new(10, 50);
    let p1 = Vec2i::new(50, 10);
    let p2 = Vec2i::new(80, 80);

    raster::triangle(p0, p1, p2, &mut image, &image::Color{r: 255, g: 0, b: 255, a: 255});
    image.save_as_ppm("output\\output.ppm").expect("Failed to save image");
}