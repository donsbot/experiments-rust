extern crate image;
extern crate num;

use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::fs::File;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT");
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

#[allow(dead_code)]
fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    match s.find(sep) {
        None => None,
        Some(i) => match (T::from_str(&s[..i]), T::from_str(&s[i + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[allow(dead_code)]
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_pair() {
    let sep = ',';
    assert_eq!(parse_pair::<i32>("", sep), None);
    assert_eq!(parse_pair::<i32>("10", sep), None);
    assert_eq!(parse_pair::<i32>(",10", sep), None); // no missing
    assert_eq!(parse_pair::<i32>("10,20", sep), Some((10, 20)));
    assert_ne!(parse_pair::<i32>("10,20", sep), Some((10, 10))); // equality
    assert_eq!(parse_pair::<i32>("10,20xy", sep), None); // no trailing
}

#[allow(dead_code)]
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

#[allow(dead_code)]
fn complex_square_root(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

#[allow(dead_code)]
fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}

#[allow(dead_code)]
fn square_loop(mut x: f64) {
    loop {
        x = x * x;
    }
}

#[allow(dead_code)]
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (w, h) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * w / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * h / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}
