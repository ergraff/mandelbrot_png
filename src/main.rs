use num::complex::Complex;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

type PIXEL = Vec<u8>;

const RESOLUTION: usize = 1000;
const ITERATIONS: usize = 100;
const LIMIT: f64 = 2.0;

struct Image {
    image: Vec<Vec<PIXEL>>,
}

impl Image {
    fn empty() -> Self {
        Image {
            image: vec![vec![vec![255, 255, 255]; RESOLUTION]; RESOLUTION],
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

fn main() {
    // Initialise image I/O
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();

    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, RESOLUTION as u32, RESOLUTION as u32);
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
    let mut image = Image::empty();
    let step: f64 = 4.0 / RESOLUTION as f64;
    for re in 0..RESOLUTION {
        for im in 0..RESOLUTION {
            let r = -2.0 + (re as f64) * step;
            let i = 2.0 - (im as f64) * step;

            let mut z = Complex::new(0.0, 0.0);
            let c = Complex::new(r, i);

            for k in 0..ITERATIONS {
                z = z * z + c;
                let unbounded = z.norm() > LIMIT;
                let bounded = (k == ITERATIONS - 1) && !unbounded;
                if unbounded {
                    image.image[im][re] = vec![255, 255, 255];
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
