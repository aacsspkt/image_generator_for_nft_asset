use font_loader::{system_fonts, system_fonts::FontPropertyBuilder};
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing;
use num_complex::Complex;
use rusttype::{Font, Scale};

fn main() {
    let start: u8 = 4;
    let count: u8 = 2;
    generate_img(start, count)
}

fn generate_img(start: u8, count: u8) {
    for i in start..(start + count) {
        let imgx = 512;
        let imgy = 512;
        let mut imgbuf = RgbImage::new(imgx, imgy);
        set_background(&mut imgbuf);

        // scale for fractals
        let scalex = 3.0 / imgx as f32;
        let scaley = 3.0 / imgy as f32;
        draw_fractal(&mut imgbuf, scalex, scaley);

        let property = FontPropertyBuilder::new().family("Arial").build();
        let (v, _) = system_fonts::get(&property).unwrap();
        let font = Font::try_from_vec(v).unwrap();
        let scale = Scale { x: 200.0, y: 200.0 };
        let text = i.to_string();

        drawing::draw_text(
            &imgbuf,
            Rgb([255, 0, 0]),
            50,
            300,
            scale,
            &font,
            text.as_str(),
        )
        .save(text + ".png")
        .unwrap();
    }
}

// add diagonal gradient of blue and red
fn set_background<'a>(img: &'a mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }
}

// draw fractal
fn draw_fractal<'a>(img: &'a mut ImageBuffer<Rgb<u8>, Vec<u8>>, scalex: f32, scaley: f32) {
    for x in 0..img.width() {
        for y in 0..img.height() {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = Complex::new(-0.4, 0.6);
            let mut z = Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = img.get_pixel_mut(x, y);
            let Rgb(data) = *pixel;
            *pixel = Rgb([data[0], i as u8, data[2]]);
        }
    }
}
