use crate::geometry::{Vec2i, Vec3i, Vec3f};
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

fn face_normal(v0: Vec3f, v1: Vec3f, v2: Vec3f) -> Vec3f {
    (v1 - v0).cross(v2 - v0)
}

fn lambert_intensity(normal: Vec3f, light_dir: Vec3f) -> f32 {
    normal.normalize().dot(light_dir.normalize()).max(0.0)
}

fn shade_from_intensity(intensity: f32) -> Color {
    let i = (intensity * 255.0) as u8;
    Color{r: i, g : i, b: i, a: 255}
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

    for i in 0..model.nfaces() {
        let face = model.face(i)?;

        let v0 = model.vert(face[0])?;
        let v1 = model.vert(face[1])?;
        let v2 = model.vert(face[2])?;
        let light_dir = Vec3f::new(-30.0,25.0, -55.0);

        let intensity = lambert_intensity(face_normal(v0 , v1, v2), light_dir);

        if intensity - 0.0 <= f32::EPSILON {
            continue;
        }

        let color = shade_from_intensity(intensity);

        let p0 = viewport(v0, width, height);
        let p1 = viewport(v1, width, height);
        let p2 = viewport(v2, width, height);

        raster::triangle(p0, p1, p2, &mut depth_buffer, &mut image, &color);
    }

    image.save_as_ppm("output\\output.ppm").map_err(|e| format!("Failed to save image: {e}"))?;
    depth_buffer.save_as_ppm("output\\depth_output.ppm").map_err(|e| format!("Failed to save image: {e}"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
        (a - b).abs() <= eps
    }

    #[test]
    fn face_normal_direction_and_length_are_correct() {
        let v0 = Vec3f::new(0.0, 0.0, 0.0);
        let v1 = Vec3f::new(1.0, 0.0, 0.0);
        let v2 = Vec3f::new(0.0, 1.0, 0.0);

        let n = face_normal(v0, v1, v2);
        let n_norm = n.normalize();

        assert!(approx_eq(n_norm.x, 0.0, 1e-6));
        assert!(approx_eq(n_norm.y, 0.0, 1e-6));
        assert!(approx_eq(n_norm.z, 1.0, 1e-6));
        assert!(approx_eq(n_norm.dot(n_norm), 1.0, 1e-6));
    }

    #[test]
    fn lambert_intensity_parallel_perpendicular_opposite() {
        let n = Vec3f::new(0.0, 0.0, 1.0);

        let parallel = lambert_intensity(n, Vec3f::new(0.0, 0.0, 3.0));
        let perpendicular = lambert_intensity(n, Vec3f::new(1.0, 0.0, 0.0));
        let opposite = lambert_intensity(n, Vec3f::new(0.0, 0.0, -2.0));

        assert!(approx_eq(parallel, 1.0, 1e-6));
        assert!(approx_eq(perpendicular, 0.0, 1e-6));
        assert!(approx_eq(opposite, 0.0, 1e-6));
    }
}
