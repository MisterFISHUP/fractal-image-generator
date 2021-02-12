extern crate image;
extern crate num_complex;

fn fractal(c1: f32, c2: f32) {
    let imgx = 800;
    let imgy = 800;
    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(c1, c2);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    imgbuf.save(format!("fractal_{}_{}.png", c1, c2)).unwrap();
}
fn main() {
    fractal(0.1, 0.6);
    fractal(0.0, 0.6);
    fractal(-0.1, 0.6);
    fractal(-0.2, 0.6);
    fractal(-0.3, 0.6);
    fractal(-0.4, 0.6);
    fractal(-0.5, 0.6);
}
