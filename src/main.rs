use crate::geometry::{Vec3i, Vec3f};
use crate::image::{Color, Image};
use crate::model::Model;

mod geometry;
mod image;
mod raster;
mod model;

fn viewport(p: Vec3f, w: usize, h: usize) -> Vec3i {
    Vec3i::new(
        ((p.x + 1.0) * w as f32 / 2.0) as i32,
        ((p.y + 1.0) * h as f32 / 2.0) as i32,
        ((p.z + 1.0) * 255.0 / 2.0) as i32)
}

fn main() -> Result<(), String> {
    let width = 100;
    let height = 100;

    let mut image = image::Image::new(width, height);
    let mut depth_buffer = Image::new(width, height);
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            depth_buffer.set(x, y, Color { r: 255, g: 255, b: 255, a: 255 });
        }
    }

    let model = Model::load_obj("asserts\\african_head.obj")?;

    let light_dir = Vec3f::new(-30.0,25.0, -55.0);
    for i in 0..model.nfaces() {
        let face = model.face(i)?;

        let v0 = model.vert(face[0].v)?;
        let v1 = model.vert(face[1].v)?;
        let v2 = model.vert(face[2].v)?;

        let n0 = face[0].vn.and_then(|vn| model.normal(vn).ok());
        let n1 = face[1].vn.and_then(|vn| model.normal(vn).ok());
        let n2 = face[2].vn.and_then(|vn| model.normal(vn).ok());

        let p0 = viewport(v0, width, height);
        let p1 = viewport(v1, width, height);
        let p2 = viewport(v2, width, height);

        raster::triangle(
            p0,
            p1,
            p2,
            v0,
            v1,
            v2,
            n0,
            n1,
            n2,
            light_dir,
            &mut depth_buffer,
            &mut image,
        );
    }

    image.save_as_ppm("output\\output.ppm").map_err(|e| format!("Failed to save image: {e}"))?;
    depth_buffer.save_as_ppm("output\\depth_output.ppm").map_err(|e| format!("Failed to save image: {e}"))?;

    Ok(())
}
