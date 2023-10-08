use crossbeam::thread::Scope;
use crossbeam;
use image::codecs::png::PngEncoder;
use image::ColorType;
use image::ImageEncoder;
use num::Complex;
use std::env;
use std::fs::File;
use std::str::FromStr;

// TODO: Refresh complex plain

fn main() {
    let cpus = num_cpus::get_physical();
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <file> <dimentions> <upper_left> <lower_right>", args[0]);
        eprintln!("Example: {} mandelbrot.png 1000x750 -1.20,0.35 -1,0.2", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing dimentions");
    let upper_left: Complex<f64> = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right: Complex<f64> = parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];
    render(&mut pixels, bounds, upper_left, lower_right, cpus);
    write_to_image(&args[1], &pixels, bounds).expect("error writting to PNG file");
}

// TODO: find out about FromStr
pub fn parse_pair<T: FromStr>(dimentions: &str, delimiter: char) -> Option<(T, T)> {
    match dimentions.find(delimiter) {
        None => None,

        Some(index) => {
            // match (dimentions[..index].parse::<usize>(), dimentions[index + 1..].parse::<usize>()) {
            match (T::from_str(&dimentions[..index]), T::from_str(&dimentions[index + 1..])) {
                (Ok(width), Ok(heigth)) => Some((width, heigth)),
                _ => None,
            }
        }
    }
}

pub fn parse_complex(range: &str) -> Option<Complex<f64>> {
    match range.find(',') {
        None => None,

        Some(index) => {
            match (range[..index].parse::<f64>(), range[index + 1..].parse::<f64>()) {
                (Ok(real), Ok(imaginary)) => Some(Complex { re: real, im: imaginary }),
                _ => None,
            }
        }
    }
}

fn render<'env>(pixels: &'env mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>, cpus: usize) {
    let rows_per_band = bounds.1 / cpus + 1;

    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        let single_render = |scope: &Scope<'env>| {
            for (i, band) in bands.into_iter().enumerate() {
                let top: usize = rows_per_band * i;
                let height: usize = band.len() / bounds.0;
                let band_bounds: (usize, usize) = (bounds.0, height);
                let band_upper_left: Complex<f64> = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right: Complex<f64> = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                scope.spawn(move |_| { render_single(band, band_bounds, band_upper_left, band_lower_right); });
            }
        };

        crossbeam::scope(single_render).unwrap();
    }
}

fn write_to_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    let width = bounds.0 as u32;
    let height = bounds.1 as u32;
    let _ = encoder.write_image(pixels, width, height, ColorType::L8)
                   .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()));
    Ok(())
}

fn render_single(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);
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

fn escape_time(complex: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i)
        }
        z = z * z + complex;
    }

    None
}

fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[cfg(test)] 
mod specs {
    use super::*;

    #[test]
    fn parse_pair_spec() {
        assert_eq!(parse_pair::<i32>("", ','), None);
        assert_eq!(parse_pair::<i32>("10", ','), None);
        assert_eq!(parse_pair::<i32>(",10", ','), None);
        assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
        assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
        assert_eq!(parse_pair::<f64>("1.0x", 'x'), None);
        assert_eq!(parse_pair::<f64>("5.0x1.3", 'x'), Some((5.0, 1.3)));
    }

    fn parse_complex_spec() {
        assert_eq!(parse_complex("1.25,"), None);
        assert_eq!(parse_complex(",-0.0625"), None);
        assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
    }

    fn pixel_to_point_spec() {
        assert_eq!(
            pixel_to_point((100,200),(25, 175), Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 }),
            Complex { re: -0.5, im: -0.75 }
        );
    }
}
