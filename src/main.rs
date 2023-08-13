use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

type PIXEL = [u8; 3];
const RED: PIXEL = [255, 0, 0];
const GREEN: PIXEL = [0, 255, 0];
const BLUE: PIXEL = [0, 0, 255];

const WIDTH: usize = 3;
const HEIGHT: usize = 3;

struct Image {
    image: [[PIXEL; WIDTH]; HEIGHT],
}

impl Image {
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

    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
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
    let image = Image {
        image: [[RED, RED, RED], [GREEN, GREEN, GREEN], [BLUE, BLUE, BLUE]],
    };

    // Write image
    let data = image.flatten();
    writer.write_image_data(&data).unwrap();
}
