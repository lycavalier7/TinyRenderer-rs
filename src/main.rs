use crate::geometry::{Vec2i, Vec3i, Vec3f};
use crate::image::{Color, Image};

mod geometry;
mod image;
mod raster;

fn viewport(p: Vec3f, w: usize, h: usize) -> Vec3i {
    Vec3i::new(
        ((p.x + 1.0) * w as f32 / 2.0) as i32,
        ((p.y + 1.0) * h as f32 / 2.0) as i32,
        ((p.z + 1.0) * 255.0 / 2.0) as i32)
}

fn main() {
    let width = 100;
    let height = 100;

    let mut image = image::Image::new(width, height);
    let mut depth_buffer = Image::new(width, height);
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            depth_buffer.set(x, y, Color { r: 255, g: 0, b: 0, a: 255 });
        }
    }

    let p0 = viewport(Vec3f{x: -0.8, y: -0.8, z: -0.5}, width, height);
    let p1 = viewport(Vec3f{x: 0.0, y: -0.8, z: -0.2}, width, height);
    let p2 = viewport(Vec3f{x: 0.5, y: 0.5, z: 0.5}, width, height);

    let p3 = viewport(Vec3f{x: -0.5, y: 0.8, z: 1.0}, width, height);
    let p4 = viewport(Vec3f{x: 0.2, y: 0.5, z: -0.3}, width, height);
    let p5 = viewport(Vec3f{x: 0.5, y: -0.5, z: 0.7}, width, height);

    raster::triangle(p0, p1, p2, &mut depth_buffer, &mut image, &image::Color{r: 255, g: 0, b: 0, a: 255});
    raster::triangle(p3, p4, p5, &mut depth_buffer, &mut image, &image::Color{r: 0, g: 0, b: 255, a: 255});
    image.save_as_ppm("output\\output.ppm").expect("Failed to save image");
    depth_buffer.save_as_ppm("output\\depth_output.ppm").expect("Failed to save image");
}
