extern crate crossbeam;
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
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
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

    // Compute rows of image pixels concurrently

    let threads = 8;
    // Height of a band = `rows_per_band`
    // Overall image width = `bounds.0`
    // Area of a band in pixels = `rows_per_band * bounds.0`
    let rows_per_band = bounds.1 / threads + 1;

    // Compute each band on it's own thread
    {
        // Each band encloses `rows_per_band` complete rows of pixels
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                // Calculate bounding box of this bound
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .unwrap();
    }

    // The above block will exit once all threads have completed (guaranteed by
    // `crossbeam::scope`) so it is now safe to save `pixels` to a file
    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

/// Determine if `c` is in the Mandelbrot Set using at most `limit` iterations.
///
/// Returns `Some(i)` if `c` is *not* a member:
/// - `i` is the number of iterations it took for `c` to leave the circle of
/// radius 2 centered on the origin.
///
/// Returns `None` if, within `limit` iterations, `c` has not been proven to
/// not be member.
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            // Left the circle of radius 2
            return Some(i);
        }
    }

    None
}

/// Parse a coordinate pair e.g. `"400x600"`, `"1.0,0.5"`
///
/// `s` should take the form <left><sep><right> where:
/// - <sep> is specified by the `separator` argument
/// - <left> and <right> can be parsed by `T::from_str`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(i) => match (T::from_str(&s[..i]), T::from_str(&s[i + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Parse complex number from a pair of comma separated floating-point numbers.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// Returns the point on the complex plain corresponding to the position of a
/// pixel in the output image where:
/// - `bounds` = (width, height) pair of the image in pixels
/// - `pixel` = (column, row) pair of a particular pixel in the image
/// - `upper_left` and `lower_right` designate the area of the complex plane
/// covered by the image.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        // Subtraction as pixel.1 increases "going down" and imaginary
        // component increases "going up"
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
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
    )
}

/// Render a rectangle of the Mandelbrot Set into a buffer of pixels where:
/// - `pixels` = buffer of image pixels, holding one grayscale pixel per byte
/// - `bounds` = (width, height) pair of the buffex `pixels`
/// - `upper_left` and `lower_left` specify points on the complex plane
/// corresponding to the upper-left and lower-right corners of the pixel buffer.
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
            };
        }
    }
}

#[derive(Debug)]
enum WriteError {
    Io(std::io::Error),
    Encode(image::error::ImageError),
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), WriteError> {
    let output = File::create(filename).map_err(WriteError::Io)?;

    let encoder = PNGEncoder::new(output);
    encoder
        .encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8)
        .map_err(WriteError::Encode)?;
    Ok(())
}
