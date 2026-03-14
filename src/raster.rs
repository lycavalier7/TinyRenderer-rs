use crate::image::Image;
use crate::image::Color;
use crate::geometry::Vec2i;

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

pub fn triangle(p0: Vec2i, p1: Vec2i, p2: Vec2i, image: &mut Image, color: &Color) {
    if image.width == 0 || image.height == 0 {
        return;
    }

    let area = (p1 - p0).cross(p2 - p0);
    if area == 0 {
        return;
    }

    let min_x = std::cmp::max(0, std::cmp::min(std::cmp::min(p0.x, p1.x), p2.x));
    let max_x = std::cmp::min((image.width - 1) as i32, std::cmp::max(std::cmp::max(p0.x, p1.x), p2.x));
    let min_y = std::cmp::max(0, std::cmp::min(std::cmp::min(p0.y, p1.y), p2.y));
    let max_y = std::cmp::min((image.height - 1) as i32, std::cmp::max(std::cmp::max(p0.y, p1.y), p2.y));

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let p = Vec2i::new(x, y);
            let alpha = (p1 - p).cross(p2 - p);
            let beta = (p2 - p).cross(p0 - p);
            let gamma = (p0 - p).cross(p1 - p);

            let inside = if area > 0 {
                alpha >= 0 && beta >= 0 && gamma >= 0
            } else {
                alpha <= 0 && beta <= 0 && gamma <= 0
            };

            if inside {
                image.set(p.x, p.y, *color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let color = Color { r: 10, g: 200, b: 10, a: 255 };
        let p0 = Vec2i::new(2, 2);
        let p1 = Vec2i::new(12, 2);
        let p2 = Vec2i::new(4, 10);

        triangle(p0, p1, p2, &mut image, &color);

        let pts = colored_points(&image, color);
        assert!(!pts.is_empty());
        assert!(pts.contains(&(4, 4)));
        assert!(!pts.contains(&(13, 13)));
    }

    #[test]
    fn triangle_winding_order_should_match() {
        let mut ccw_image = Image::new(20, 20);
        let mut cw_image = Image::new(20, 20);
        let color = Color { r: 240, g: 10, b: 10, a: 255 };

        let p0 = Vec2i::new(3, 3);
        let p1 = Vec2i::new(15, 4);
        let p2 = Vec2i::new(6, 14);

        triangle(p0, p1, p2, &mut ccw_image, &color);
        triangle(p0, p2, p1, &mut cw_image, &color);

        assert_eq!(colored_points(&ccw_image, color), colored_points(&cw_image, color));
    }

    #[test]
    fn triangle_partially_out_of_bounds_still_rasterizes() {
        let mut image = Image::new(8, 8);
        let color = Color { r: 50, g: 100, b: 200, a: 255 };
        let p0 = Vec2i::new(-4, 2);
        let p1 = Vec2i::new(4, -3);
        let p2 = Vec2i::new(5, 6);

        triangle(p0, p1, p2, &mut image, &color);

        let pts = colored_points(&image, color);
        assert!(!pts.is_empty());
        assert!(pts.iter().all(|(x, y)| *x >= 0 && *x < 8 && *y >= 0 && *y < 8));
    }
}
