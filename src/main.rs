use num::complex::Complex;
use std::f64::consts::PI;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

type PIXEL = Vec<u8>;

const ITERATIONS: usize = 50;
const RESOLUTION: usize = 1000;
const LIMIT_BOUNDED: f64 = 2.0;
const LIMIT_RE: [f64; 2] = [-2.0, 1.0];
const LIMIT_IM: [f64; 2] = [1.0, -1.0];

struct Image {
    image: Vec<Vec<PIXEL>>,
}

impl Image {
    fn empty(height: usize, width: usize) -> Self {
        Image {
            image: vec![vec![vec![0, 0, 0]; height]; width],
        }
    }

    fn flatten(self) -> Vec<u8> {
        let mut output = vec![];
        for row in self.image {
            for pixel in row {
                for val in pixel {
                    output.push(val);
                }
            }
        }
        output
    }
}

fn pixel_color(strength: f64) -> PIXEL {
    let rads = 2.0 * PI * strength;
    let height_scaling = 255.0;
    let width_scaling = 0.5;
    let r = (height_scaling * (width_scaling * (rads - 2.0 * PI)).cos()) as u8;
    let g = (height_scaling * (width_scaling * (rads - 1.0 * PI)).cos()) as u8;
    let b = (height_scaling * (width_scaling * (rads - 0.0 * PI)).cos()) as u8;
    vec![r, g, b]
}

fn main() {
    // Initialise image I/O
    let sum_im = LIMIT_IM.map(|v| v.abs()).into_iter().sum::<f64>();
    let sum_re = LIMIT_RE.map(|v| v.abs()).into_iter().sum::<f64>();
    let width = RESOLUTION;
    let height = (RESOLUTION as f64 * sum_im / sum_re) as usize;
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();

    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let source_chromaticities = png::SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    // Create image
    let mut image = Image::empty(width, height);
    let step_re: f64 = (sum_re as f64) / width as f64;
    let step_im: f64 = (sum_im as f64) / height as f64;
    for re in 0..width {
        for im in 0..height {
            let r = LIMIT_RE[0] as f64 + (re as f64) * step_re;
            let i = LIMIT_IM[0] as f64 - (im as f64) * step_im;

            let mut z = Complex::new(0.0, 0.0);
            let c = Complex::new(r, i);

            for k in 0..ITERATIONS {
                z = z * z + c;
                let unbounded = z.norm() > LIMIT_BOUNDED;
                let bounded = (k == ITERATIONS - 1) && !unbounded;
                if unbounded {
                    let strength = k as f64 / (ITERATIONS as f64 / 2.0);
                    image.image[im][re] = pixel_color(strength);
                    break;
                }
                if bounded {
                    image.image[im][re] = vec![0, 0, 0];
                }
            }
        }
    }

    let data = image.flatten();

    // Write image
    let data = image.flatten();
    writer.write_image_data(&data).unwrap();
}
