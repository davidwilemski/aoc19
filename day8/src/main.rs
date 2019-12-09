use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let mut stdin = std::io::stdin();

    let mut bytes = vec![];
    stdin.read_to_end(&mut bytes)?;
    let digits : Vec<u32> = bytes.iter()
        .filter(|b| b.is_ascii_digit())
        .map(|b| {
            char::from(*b).to_digit(10).unwrap()
        })
    .collect();

    let layers : Vec<&[u32]> = digits.chunks(25 * 6 as usize).collect();
    let min_zeros = layers
        .iter()
        .enumerate()
        .map(|(i, layer)| {
            (i, layer.iter().filter(|v| **v == 0).count())
        })
        .min_by_key(|(_, count)| *count).unwrap();
    println!("layer with min zeros: {:?}", min_zeros);


    // On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
    let one_digits_count = layers[min_zeros.0].iter().filter(|v| **v == 1).count();
    let two_digits_count = layers[min_zeros.0].iter().filter(|v| **v == 2).count();
    let result = (one_digits_count * two_digits_count) as u32;
    println!("one digits count ({}) times two digits count ({}): {}", one_digits_count, two_digits_count, result);

    let pixel_vals = (0..(25 * 6)).map(|i| determine_pixel(i, &layers)).collect::<Vec<Pixel>>();

    for row in 0..6 {
        for col in 0..25 {
            print!("{} ", pixel_vals[row * 25 + col]);
        }
        print!("\n");
    }
    Ok(())
}

#[derive(Debug)]
enum Pixel {
    Transparent,
    Black,
    White,
}

impl std::convert::From<u32> for Pixel {
    fn from(val: u32) -> Self {
        match val {
            0 => Pixel::Black,
            1 => Pixel::White,
            2 => Pixel::Transparent,
            _ => panic!("shouldn't happen")
        }
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let val = match self {
            Pixel::Black => " ",
            Pixel::White => "1",
            Pixel::Transparent => " ",
        };
        write!(f, "{}", val)
    }
}

/// The layers are rendered with the first layer in front and the last layer in back.
/// So, find the first layer that is black or white and return that value, otherwise use the last
/// value.
fn determine_pixel(pixel_position: usize, layers: &Vec<&[u32]>) -> Pixel {
    let pixels = layers.iter().map(|layer| Pixel::from(layer[pixel_position]));

    for pixel in pixels {
        let result = match pixel {
            Pixel::Black|Pixel::White => {
                true
            },
            Pixel::Transparent => {
                false
            }
        };

        if result {
            return pixel
        }
    }
    Pixel::Transparent
}
