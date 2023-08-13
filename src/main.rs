use num::complex::Complex;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

type PIXEL = [u8; 3];
const _RED: PIXEL = [255, 0, 0];
const _GREEN: PIXEL = [0, 255, 0];
const _BLUE: PIXEL = [0, 0, 255];
const BLACK: PIXEL = [0, 0, 0];
const WHITE: PIXEL = [255, 255, 255];

const RESOLUTION: usize = 500;

const ITERATIONS: usize = 10;
const LIMIT: f64 = 2.0;

struct Image {
    image: [[PIXEL; RESOLUTION]; RESOLUTION],
}

impl Image {
    fn empty() -> Self {
        Image {
            image: [[WHITE; RESOLUTION]; RESOLUTION],
        }
    }

    fn flatten(self) -> Vec<u8> {
        let mut output = vec![];
        for row in self.image {
            for pixel in row {
                pixel.map(|v| output.push(v));
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
    let step: f64 = 4.0 / RESOLUTION as f64;
    let mut image = Image::empty();
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
                match (unbounded, bounded) {
                    (true, _) => image.image[im][re] = WHITE,
                    (_, true) => image.image[im][re] = BLACK,
                    _ => {}
                }
            }
        }
    }

    let data = image.flatten();

    // Write image
    let data = image.flatten();
    writer.write_image_data(&data).unwrap();
}
