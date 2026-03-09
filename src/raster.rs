use crate::image::Image;
use crate::geometry::Vec2;

fn line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut Image, color: u8) {
    if x0 == x1 && y0 == y1 {
        image.set(x0, y0, color);
        return;
    }

    let mut x0 = x0;
    let mut y0 = y0;
    let mut x1 = x1;
    let mut y1 = y1;

    let steep: bool = (x1 - x0).abs() < (y1 - y0).abs();

    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    if x1 < x0 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = (x1 - x0);
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

    fn colored_points(image: &Image, color: u8) -> Vec<(i32, i32)> {
        let mut pts = Vec::new();

        for y in 0..image.height as i32 {
            for x in 0..image.width as i32{
                if image.get(x,y) == color {
                    pts.push((x,y));
                }
            }
        }
        pts
    }

    #[test]
    fn line_horizontal_includes_enpoint() {
        let mut image = Image::new(8,8);

        line(1,3,4,3, &mut image, 255);

        let actual = colored_points(&image, 255);
        let expected = vec![(1,3), (2,3),(3,3),(4,3)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn line_vertical_draws_all_points() {
        let mut image = Image::new(8, 8);
        line(2, 1, 2, 4, &mut image, 200);
        assert_eq!(colored_points(&image, 200), vec![(2, 1), (2, 2), (2, 3), (2, 4)]);
    }

    #[test]
    fn line_diagonal_45_degrees() {
        let mut image = Image::new(8, 8);
        line(0, 0, 3, 3, &mut image, 180);
        assert_eq!(colored_points(&image, 180), vec![(0, 0), (1, 1), (2, 2), (3, 3)]);
    }

    #[test]
    fn line_reverse_order_same_pixels() {
        let mut image = Image::new(8, 8);
        line(4, 1, 1, 1, &mut image, 123);
        assert_eq!(colored_points(&image, 123), vec![(1, 1), (2, 1), (3, 1), (4, 1)]);
    }

    #[test]
    fn line_single_point() {
        let mut image = Image::new(8, 8);
        line(5, 6, 5, 6, &mut image, 77);
        assert_eq!(colored_points(&image, 77), vec![(5, 6)]);
    }

    #[test]
    fn line_negative_slope() {
        let mut image = Image::new(8, 8);
        line(1, 4, 4, 1, &mut image, 88);

        let mut actual = colored_points(&image, 88);
        let mut expected = vec![(1, 4), (2, 3), (3, 2), (4, 1)];
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn line_clips_out_of_bounds_via_image_set() {
        let mut image = Image::new(5, 5);
        line(-2, 2, 2, 2, &mut image, 66);
        assert_eq!(colored_points(&image, 66), vec![(0, 2), (1, 2), (2, 2)]);
    }
}