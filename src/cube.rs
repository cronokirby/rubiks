const ANSI_WHITE: &'static str = "\x1b[37m";
const ANSI_RED: &'static str = "\x1b[31m";
const ANSI_GREEN: &'static str = "\x1b[32m";
const ANSI_MAGENTA: &'static str = "\x1b[35m";
const ANSI_BLUE: &'static str = "\x1b[34m";
const ANSI_YELLOW: &'static str = "\x1b[33m";
const ANSI_RESET: &'static str = "\x1b[0m";

/// A color we can see on a cube.
///
/// This corresponds both to the color of a facelet / sticker, as well as
/// to the color of a side, which is a bit more correct geometrically.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum Color {
    White = 0,
    Red = 1,
    Green = 2,
    Orange = 3,
    Blue = 4,
    Yellow = 5,
}

impl Color {
    fn print(&self) {
        match self {
            Color::White => print!("{}■{}", ANSI_WHITE, ANSI_RESET),
            Color::Red => print!("{}■{}", ANSI_RED, ANSI_RESET),
            Color::Green => print!("{}■{}", ANSI_GREEN, ANSI_RESET),
            // Well, there's no orange, so we gotta use magenta
            Color::Orange => print!("{}■{}", ANSI_MAGENTA, ANSI_RESET),
            Color::Blue => print!("{}■{}", ANSI_BLUE, ANSI_RESET),
            Color::Yellow => print!("{}■{}", ANSI_YELLOW, ANSI_RESET),
        }
    }
}

/// A face contains all the "stickers" we can see looking at the cube from a direction.
///
/// Strictly speaking, a face doesn't really "exist", because a rubik's cube's individual pieces
/// can't be separated. A common conception about how the cube works is that it's composed of six faces
/// with 9 stickers, and the stickers move arounde freely. This representaiton is convenient, but can
/// represent invalid states.
///
/// The stickers are in row major format. The first row is defined as that which is closest to the white
/// face. For the white face, instead we use the row closest to the green face.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Face([Color; 9]);

/// This represents a rubik's cube.
///
/// This representation is based on the misconception that all stickers on a cube move around freely.
/// This is a convenient representation to work with, as it's easy to keep track of the visual appearance
/// of the cube. It's also a simple representation to define.
///
/// The biggest disadvantage is that some invalid states are possible.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cube {
    /// This maps each color to its corresponding face (defined by the centerpiece).
    ///
    /// The indices are defined by casting colors.
    faces: [Face; 6],
}

impl Cube {
    pub fn solved() -> Self {
        Cube {
            faces: [
                Face([Color::White; 9]),
                Face([Color::Red; 9]),
                Face([Color::Green; 9]),
                Face([Color::Orange; 9]),
                Face([Color::Blue; 9]),
                Face([Color::Yellow; 9]),
            ],
        }
    }

    pub fn print(&self) {
        for Face(arr) in &self.faces {
            for y in 0..3 {
                for x in 0..3 {
                    arr[y * 3 + x].print()
                }
                print!("\n");
            }
        }
    }
}
