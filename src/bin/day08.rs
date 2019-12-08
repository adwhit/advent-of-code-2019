#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pixel {
    White,
    Black,
    Transparant,
}

impl From<u8> for Pixel {
    fn from(val: u8) -> Pixel {
        match val {
            0 => Pixel::Black,
            1 => Pixel::White,
            2 => Pixel::Transparant,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add for Pixel {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Pixel::Transparant, other) => other,
            (me, _) => me,
        }
    }
}

impl std::ops::AddAssign for Pixel {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pixel::Black => write!(f, " "),
            Pixel::White => write!(f, "#"),
            Pixel::Transparant => write!(f, " "),
        }
    }
}

fn read(width: usize, height: usize) -> Vec<Vec<Vec<Pixel>>> {
    let s = std::fs::read_to_string("data/08").unwrap();
    s.trim()
        .as_bytes()
        .chunks(width * height)
        .map(|layer| {
            layer
                .chunks(width)
                .map(|line| line.iter().map(|byte| Pixel::from(byte - b'0')).collect())
                .collect()
        })
        .collect()
}

fn layer_count_byte(layer: &[Vec<Pixel>], val: Pixel) -> usize {
    layer
        .iter()
        .map(|line| line.iter().filter(|v| **v == val).count())
        .sum()
}

fn render(layers: &[Vec<Vec<Pixel>>]) -> Vec<Vec<Pixel>> {
    let mut out = vec![vec![Pixel::Transparant; 25]; 6];
    for layer in layers {
        for (lix, line) in layer.iter().enumerate() {
            for (pix, elem) in line.iter().enumerate() {
                out[lix][pix] += *elem
            }
        }
    }
    out
}

fn view(img: &[Vec<Pixel>]) {
    for line in img {
        for elem in line {
            print!("{}", elem);
        }
        println!()
    }
}

fn main() {
    let img = read(25, 6);
    let (minix, _minval) = img
        .iter()
        .map(|layer| layer_count_byte(&*layer, Pixel::Black))
        .enumerate()
        .fold(
            (std::usize::MAX, std::usize::MAX),
            |(accix, accval), (ix, val)| {
                if val < accval {
                    (ix, val)
                } else {
                    (accix, accval)
                }
            },
        );
    let ones = layer_count_byte(&img[minix], Pixel::White);
    let twos = layer_count_byte(&img[minix], Pixel::Transparant);
    println!("{}: {}", minix, ones * twos);

    let rendered = render(&img);
    view(&rendered);
}
