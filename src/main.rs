mod geometry;
mod image;
mod raster;

fn main() {
    let x0 = 10;
    let y0 = 30;
    let x1 = 50;
    let y1 = 70;
    let width = 100;
    let height = 100;

    let mut image = image::Image::new(width, height);

    raster::line(x0, y0, x1, y1, &mut image, 128);

    image.save_as_pgm("output.pgm").expect("Failed to save image");
}