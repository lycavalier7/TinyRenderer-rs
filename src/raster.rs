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

#[cfg(test)]
mod tests {
    use super::*;

    fn colored_points(image: &Image, color: Color) -> Vec<(i32, i32)> {
        let mut pts = Vec::new();

        for y in 0..image.height as i32 {
            for x in 0..image.width as i32{
                if image.get(x,y).r == color.r &&
                    image.get(x,y).g == color.g &&
                    image.get(x,y).b == color.b &&
                    image.get(x,y).a == color.a{
                    pts.push((x,y));
                }
            }
        }
        pts
    }

    #[test]
    fn line_horizontal_includes_enpoint() {
        let mut image = Image::new(8,8);
        let color = Color{r: 255, g: 255, b: 255, a: 255};
        let point0 = Vec2i::new(1,3);
        let point1 = Vec2i::new(4,3);

        line(point0, point1, &mut image, color);

        let actual = colored_points(&image, color);
        let expected = vec![(1,3), (2,3),(3,3),(4,3)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn line_vertical_draws_all_points() {
        let mut image = Image::new(8, 8);
        let color = Color{r: 200, g: 200, b: 200, a: 255};
        let point0 = Vec2i::new(2,1);
        let point1 = Vec2i::new(2,4);

        line(point0, point1, &mut image, color);
        assert_eq!(colored_points(&image, color), vec![(2, 1), (2, 2), (2, 3), (2, 4)]);
    }

    #[test]
    fn line_diagonal_45_degrees() {
        let mut image = Image::new(8, 8);
        let color = Color{r: 180, g: 180, b: 180, a: 255};
        let point0 = Vec2i::new(0,0);
        let point1 = Vec2i::new(3,3);

        line(point0, point1, &mut image, color);
        assert_eq!(colored_points(&image, color), vec![(0, 0), (1, 1), (2, 2), (3, 3)]);
    }

    #[test]
    fn line_reverse_order_same_pixels() {
        let mut image = Image::new(8, 8);
        let color = Color{r: 123, g: 123, b: 123, a: 255};
        let point0 = Vec2i::new(4,1);
        let point1 = Vec2i::new(1,1);

        line(point0, point1, &mut image, color);
        assert_eq!(colored_points(&image, color), vec![(1, 1), (2, 1), (3, 1), (4, 1)]);
    }

    #[test]
    fn line_single_point() {
        let mut image = Image::new(8, 8);
        let color = Color{r: 77, g: 77, b: 77, a: 255};
        let point0 = Vec2i::new(5,6);
        let point1 = Vec2i::new(5,6);

        line(point0, point1, &mut image, color);
        assert_eq!(colored_points(&image, color), vec![(5, 6)]);
    }

    #[test]
    fn line_negative_slope() {
        let mut image = Image::new(8, 8);
        let color = Color{r: 88, g: 88, b: 88, a: 255};
        let point0 = Vec2i::new(1,4);
        let point1 = Vec2i::new(4,1);

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
        let color = Color{r: 66, g: 66, b: 66, a: 255};
        let point0 = Vec2i::new(-2,2);
        let point1 = Vec2i::new(2,2);

        line(point0, point1, &mut image, color);
        assert_eq!(colored_points(&image, color), vec![(0, 2), (1, 2), (2, 2)]);
    }
}