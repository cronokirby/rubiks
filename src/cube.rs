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

/// A face contains all the "stickers" we can see looking at the cube from a direction.
///
/// Strictly speaking, a face doesn't really "exist", because a rubik's cube's individual pieces
/// can't be separated. A common conception about how the cube works is that it's composed of six faces
/// with 9 stickers, and the stickers move arounde freely. This representaiton is convenient, but can
/// represent invalid states.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Face {
    /// The stickers, in row major format.
    ///
    /// The first row is defined as that which is closest to the white face, and the row closest
    /// to the green face for white.
    stickers: [Color; 9],
}

/// This represents a rubik's cube.
///
/// This representation is based on the misconception that all stickers on a cube move around freely.
/// This is a convenient representation to work with, as it's easy to keep track of the visual appearance
/// of the cube. It's also a simple representation to define.
///
/// The biggest disadvantage is that some invalid states are possible.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Cube {
    /// This maps each color to its corresponding face (defined by the centerpiece).
    ///
    /// The indices are defined by casting colors.
    faces: [Face; 6],
}
