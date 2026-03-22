use crate::image::Image;
use crate::image::Color;
use crate::geometry::{Vec2i, Vec3f};
use crate::geometry::Vec3i;

pub fn line(point0: Vec2i, point1: Vec2i, image: &mut Image, color: Color) {
    let mut x0 = point0.x;
    let mut y0 = point0.y;
    let mut x1 = point1.x;
    let mut y1 = point1.y;

    if x0 == x1 && y0 == y1 {
        image.set(x0, y0, color);
        return;
    }

    let steep: bool = (x1 - x0).abs() < (y1 - y0).abs();

    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    if x1 < x0 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = (y1 - y0).abs();

    let mut error2 = 0;
    let mut y = y0;
    let y_step = if y1 > y0 { 1} else { -1 };

    for x in x0..=x1 {
        if steep {
            image.set(y, x, color);
        } else{
            image.set(x, y, color)
        }

        error2 += dy * 2;

        if error2 > dx {
            y += y_step;
            error2 -= dx * 2;
        }
    }
}

fn face_normal(
    v0: Vec3f,
    v1: Vec3f,
    v2: Vec3f,
    n0: Option<Vec3f>,
    n1: Option<Vec3f>,
    n2: Option<Vec3f>,
    bary: (f32, f32, f32)
   ) -> Vec3f {
    match (n0, n1, n2) {
        (Some(n0), Some(n1), Some(n2)) => {
            let (alpha, beta, gamma) = bary;
            (n0 * alpha + n1 * beta + n2 * gamma).normalize()
        }
        _ => {
            (v1 - v0).cross(v2 - v0).normalize()
        }
    }
}

fn lambert_intensity(normal: Vec3f, light_dir: Vec3f) -> f32 {
    normal.normalize().dot(light_dir.normalize()).max(0.0)
}

fn shade_from_intensity(intensity: f32) -> Color {
    let i = (intensity * 255.0) as u8;
    Color{r: i, g : i, b: i, a: 255}
}

pub fn triangle(
    p0: Vec3i,
    p1: Vec3i,
    p2: Vec3i,
    v0: Vec3f,
    v1: Vec3f,
    v2: Vec3f,
    n0: Option<Vec3f>,
    n1: Option<Vec3f>,
    n2: Option<Vec3f>,
    light_dir: Vec3f,
    depth_buffer: &mut Image,
    framebuffer: &mut Image,
) {
    if framebuffer.width == 0 || framebuffer.height == 0 {
        return;
    }

    let point0: Vec2i = Vec2i::new(p0.x, p0.y);
    let point1: Vec2i = Vec2i::new(p1.x, p1.y);
    let point2: Vec2i = Vec2i::new(p2.x, p2.y);

    let area = (point1 - point0).cross(point2 - point0);
    if area == 0 {
        return;
    }

    let min_x = std::cmp::max(0, std::cmp::min(std::cmp::min(p0.x, p1.x), p2.x));
    let max_x = std::cmp::min((framebuffer.width - 1) as i32, std::cmp::max(std::cmp::max(p0.x, p1.x), p2.x));
    let min_y = std::cmp::max(0, std::cmp::min(std::cmp::min(p0.y, p1.y), p2.y));
    let max_y = std::cmp::min((framebuffer.height - 1) as i32, std::cmp::max(std::cmp::max(p0.y, p1.y), p2.y));

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let p = Vec2i::new(x, y);
            let alpha = (point1 - p).cross(point2 - p);
            let beta = (point2 - p).cross(point0 - p);
            let gamma = (point0 - p).cross(point1 - p);

            let z_num = alpha * p0.z + beta * p1.z + gamma * p2.z;
            let z: i32 = z_num / area;

            let inside = if area > 0 {
                alpha >= 0 && beta >= 0 && gamma >= 0
            } else {
                alpha <= 0 && beta <= 0 && gamma <= 0
            };

            let cull = if z <= depth_buffer.get(p.x, p.y).r as i32 {
                true
            } else {
                false
            };

            if inside && cull == false{
                let bary = (
                    alpha as f32 / area as f32,
                    beta as f32 / area as f32,
                    gamma as f32 / area as f32,
                );
                let normal = face_normal(v0, v1, v2, n0, n1, n2, bary);
                let intensity = lambert_intensity(normal, light_dir);
                if intensity <= f32::EPSILON {
                    continue;
                }
                let color = shade_from_intensity(intensity);
                framebuffer.set(p.x, p.y, color);
                depth_buffer.set(p.x, p.y, Color{r: z as u8, g: z as u8, b: z as u8, a: 255});
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
        (a - b).abs() <= eps
    }

    fn colored_points(image: &Image, color: Color) -> Vec<(i32, i32)> {
        let mut pts = Vec::new();

        for y in 0..image.height as i32 {
            for x in 0..image.width as i32 {
                if image.get(x, y).r == color.r &&
                    image.get(x, y).g == color.g &&
                    image.get(x, y).b == color.b &&
                    image.get(x, y).a == color.a {
                    pts.push((x, y));
                }
            }
        }
        pts
    }

    #[test]
    fn face_normal_direction_and_length_are_correct() {
        let v0 = Vec3f::new(0.0, 0.0, 0.0);
        let v1 = Vec3f::new(1.0, 0.0, 0.0);
        let v2 = Vec3f::new(0.0, 1.0, 0.0);

        let n = face_normal(v0, v1, v2, None, None, None, (1.0, 0.0, 0.0));
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

    #[test]
    fn line_horizontal_includes_enpoint() {
        let mut image = Image::new(8, 8);
        let color = Color { r: 255, g: 255, b: 255, a: 255 };
        let point0 = Vec2i::new(1, 3);
        let point1 = Vec2i::new(4, 3);

        line(point0, point1, &mut image, color);

        let actual = colored_points(&image, color);
        let expected = vec![(1, 3), (2, 3), (3, 3), (4, 3)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn line_vertical_draws_all_points() {
        let mut image = Image::new(8, 8);
        let color = Color { r: 200, g: 200, b: 200, a: 255 };
        let point0 = Vec2i::new(2, 1);
        let point1 = Vec2i::new(2, 4);

        line(point0, point1, &mut image, color);
        assert_eq!(colored_points(&image, color), vec![(2, 1), (2, 2), (2, 3), (2, 4)]);
    }

    #[test]
    fn line_diagonal_45_degrees() {
        let mut image = Image::new(8, 8);
        let color = Color { r: 180, g: 180, b: 180, a: 255 };
        let point0 = Vec2i::new(0, 0);
        let point1 = Vec2i::new(3, 3);

        line(point0, point1, &mut image, color);
        assert_eq!(colored_points(&image, color), vec![(0, 0), (1, 1), (2, 2), (3, 3)]);
    }

    #[test]
    fn line_reverse_order_same_pixels() {
        let mut image = Image::new(8, 8);
        let color = Color { r: 123, g: 123, b: 123, a: 255 };
        let point0 = Vec2i::new(4, 1);
        let point1 = Vec2i::new(1, 1);

        line(point0, point1, &mut image, color);
        assert_eq!(colored_points(&image, color), vec![(1, 1), (2, 1), (3, 1), (4, 1)]);
    }

    #[test]
    fn line_single_point() {
        let mut image = Image::new(8, 8);
        let color = Color { r: 77, g: 77, b: 77, a: 255 };
        let point0 = Vec2i::new(5, 6);
        let point1 = Vec2i::new(5, 6);

        line(point0, point1, &mut image, color);
        assert_eq!(colored_points(&image, color), vec![(5, 6)]);
    }

    #[test]
    fn line_negative_slope() {
        let mut image = Image::new(8, 8);
        let color = Color { r: 88, g: 88, b: 88, a: 255 };
        let point0 = Vec2i::new(1, 4);
        let point1 = Vec2i::new(4, 1);

        line(point0, point1, &mut image, color);

        let mut actual = colored_points(&image, color);
        let mut expected = vec![(1, 4), (2, 3), (3, 2), (4, 1)];
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn line_clips_out_of_bounds_via_image_set() {
        let mut image = Image::new(5, 5);
        let color = Color { r: 66, g: 66, b: 66, a: 255 };
        let point0 = Vec2i::new(-2, 2);
        let point1 = Vec2i::new(2, 2);

        line(point0, point1, &mut image, color);
        assert_eq!(colored_points(&image, color), vec![(0, 2), (1, 2), (2, 2)]);
    }

    #[test]
    fn triangle_filled_draws_pixels_inside_bbox() {
        let mut image = Image::new(16, 16);
        let mut depth = Image::new(16, 16);
        let p0 = Vec3i::new(2, 2, 1);
        let p1 = Vec3i::new(12, 2, 1);
        let p2 = Vec3i::new(4, 10, 1);
        let v0 = Vec3f::new(0.0, 0.0, 0.0);
        let v1 = Vec3f::new(1.0, 0.0, 0.0);
        let v2 = Vec3f::new(0.0, 1.0, 0.0);
        let n = Some(Vec3f::new(0.0, 0.0, 1.0));
        let light = Vec3f::new(0.0, 0.0, 1.0);
        let lit = Color { r: 255, g: 255, b: 255, a: 255 };

        triangle(p0, p1, p2, v0, v1, v2, n, n, n, light, &mut depth, &mut image);

        let pts = colored_points(&image, lit);
        assert!(!pts.is_empty());
        assert!(pts.contains(&(4, 4)));
        assert!(!pts.contains(&(13, 13)));
    }

    #[test]
    fn triangle_winding_order_should_match() {
        let mut ccw_image = Image::new(20, 20);
        let mut cw_image = Image::new(20, 20);
        let mut ccw_depth = Image::new(20, 20);
        let mut cw_depth = Image::new(20, 20);

        let p0 = Vec3i::new(3, 3, 1);
        let p1 = Vec3i::new(15, 4, 1);
        let p2 = Vec3i::new(6, 14, 1);
        let v0 = Vec3f::new(0.0, 0.0, 0.0);
        let v1 = Vec3f::new(1.0, 0.0, 0.0);
        let v2 = Vec3f::new(0.0, 1.0, 0.0);
        let n = Some(Vec3f::new(0.0, 0.0, 1.0));
        let light = Vec3f::new(0.0, 0.0, 1.0);
        let lit = Color { r: 255, g: 255, b: 255, a: 255 };

        triangle(p0, p1, p2, v0, v1, v2, n, n, n, light, &mut ccw_depth, &mut ccw_image);
        triangle(p0, p2, p1, v0, v2, v1, n, n, n, light, &mut cw_depth, &mut cw_image);

        assert_eq!(colored_points(&ccw_image, lit), colored_points(&cw_image, lit));
    }

    #[test]
    fn triangle_partially_out_of_bounds_still_rasterizes() {
        let mut image = Image::new(8, 8);
        let mut depth = Image::new(8, 8);
        let p0 = Vec3i::new(-4, 2, 1);
        let p1 = Vec3i::new(4, -3, 1);
        let p2 = Vec3i::new(5, 6, 1);
        let v0 = Vec3f::new(0.0, 0.0, 0.0);
        let v1 = Vec3f::new(1.0, 0.0, 0.0);
        let v2 = Vec3f::new(0.0, 1.0, 0.0);
        let n = Some(Vec3f::new(0.0, 0.0, 1.0));
        let light = Vec3f::new(0.0, 0.0, 1.0);
        let lit = Color { r: 255, g: 255, b: 255, a: 255 };

        triangle(p0, p1, p2, v0, v1, v2, n, n, n, light, &mut depth, &mut image);

        let pts = colored_points(&image, lit);
        assert!(!pts.is_empty());
        assert!(pts.iter().all(|(x, y)| *x >= 0 && *x < 8 && *y >= 0 && *y < 8));
    }
}
