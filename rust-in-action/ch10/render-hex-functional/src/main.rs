//! # render-hex - SVG rendering of hex data
//!
//! Note: This is a modified version of `render-hex` whereby the `parse`
//! function uses a functional iterator style instead of explicit `for` loop
//! in *preparation* for parallelisation using `rayon`.
//!
//! Generates and SVG representation of arbitrary hexadecimal input by
//! converting the hexadecimal digits into a set of drawing instructions.
//!
//! ## Usage example
//!
//! ```sh
//! $ cargo run -- $(echo 'Hello, world!' | sha256sum | cut -f1 -d' ')
//! $ cat d9014c4624844aa5bac314773d6b689ad467fa4e1d1a50a1b8a99d5a95f72ff5.svg
//! <svg height="400" style='style="outline: 5px solid #800000;"' viewBox="0 0 400 400" width="400" xmlns="http://www.w3.org/2000/svg">
//! <rect fill="#ffffff" height="400" width="400" x="0" y="0"/>
//! <path d="M200,200 L200,200 L-160,200 L200,200 L240,200 L400,200 L400,200 L400,40 L400,-200 L400,280 L400,440 L400,-120 L400,360 L400,520 L400,200 L400,200 L400,400 L400,400 L400,400 L400,400 L280,400 L240,400 L80,400 L-200,400 L480,400 L80,400 L80,400 L80,160 L80,160 L-160,160 L520,160 L-160,160 L200,160 L200,160 L360,160 L600,160 L-80,160 L200,160 L200,160 L360,160 L360,160 L360,200 L360,200 L320,200 L320,200 L320,400 L200,200 L200,200 L240,200 L240,200 L240,-120 L240,200 L600,200 L-160,200 L200,200 L200,400 L200,400 L560,400 L0,400 L0,400 L0,120 L0,40 L0,40 L0,40 L0,240" fill="none" stroke="#2f2f2f" stroke-opacity="0.9" stroke-width="5"/>
//! <rect fill="#ffffff" fill-opacity="0.0" height="400" stroke="#cccccc" stroke-width="15" width="400" x="0" y="0"/>
//! </svg>
//! ```
use crate::{Operation::*, Orientation::*};
use std::env;
use svg::{
    node::element::{
        path::{Command, Data, Position},
        Path, Rectangle,
    },
    Document,
};

// Bounds for drawing
const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;

// Home coordinates (center of canvas)
const HOME_X: isize = WIDTH / 2;
const HOME_Y: isize = HEIGHT / 2;

const STROKE_WIDTH: usize = 5;

const ASCII_NUMERAL_START: u8 = 0x30;

/// Drawing instructions. The term "operation" is used here to distinguish
/// from SVG path drawing terminology.
#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    North,
    East,
    West,
    South,
}

/// Maintains the state of the diagram; at any given time the "artist" is
/// 'holding' a pen at coordinate `(x, y)` and moving in direction of `heading`.
#[derive(Debug)]
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Artist {
    /// Initialise a new `Artist` facing `North` at the home location.
    fn new() -> Artist {
        Artist {
            heading: North,
            x: HOME_X,
            y: HOME_Y,
        }
    }

    /// Move the artist to the home location.
    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    /// Move the artist `distance` units in the direction of it's current
    /// `heading`.
    fn forward(&mut self, distance: isize) {
        match self.heading {
            North => self.y += distance,
            South => self.y -= distance,
            West => self.x += distance,
            East => self.x -= distance,
        }
    }

    fn turn_right(&mut self) {
        self.heading = match self.heading {
            North => East,
            South => West,
            West => North,
            East => South,
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            North => West,
            South => East,
            West => South,
            East => North,
        }
    }

    /// Alters position and heading to ensure the artist stays within the
    /// bounds of the canvas.
    fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = North;
        } else if self.y > HEIGHT {
            self.y = HOME_Y;
            self.heading = South;
        }
    }
}

/// Parse input bytes into a sequence of `Operation`s:
/// - Numerals are converted to movement distances
/// - Letters change the orientation of drawing
fn parse(input: &str) -> Vec<Operation> {
    input
        .bytes()
        .map(|byte| {
            match byte {
                b'0' => Home,
                b'1'..=b'9' => {
                    // Convert ASCII numerals to numeric values
                    // Safe from panics as the range of values is restricted by the
                    // match clause (whole u8 range would panic)
                    let distance = (byte - ASCII_NUMERAL_START) as isize;
                    Forward(distance * (HEIGHT / 10))
                }
                // TODO: Extract into a configurable table?
                // Can add more instructions here to change drawing behaviour
                b'a' | b'b' | b'c' => TurnLeft,
                b'd' | b'e' | b'f' => TurnRight,
                // Allow the caller to handle invalid characters
                // - Decouple parsing from producing output
                _ => Noop(byte),
            }
        })
        .collect()
}

/// Convert a sequence of `Operation`s into a sequence of SVG `Command`s for
/// SVG generation.
fn convert(operations: &Vec<Operation>) -> Vec<Command> {
    // LOGO :D
    let mut turtle = Artist::new();

    let mut path_data = Vec::<Command>::with_capacity(1 + operations.len());
    // Start at home location
    path_data.push(Command::Move(Position::Absolute, (HOME_X, HOME_Y).into()));

    for op in operations {
        // Update turtle state
        match *op {
            Forward(distance) => turtle.forward(distance),
            TurnLeft => turtle.turn_left(),
            TurnRight => turtle.turn_right(),
            Home => turtle.home(),
            Noop(byte) => eprintln!("warning: illegal byte encountered: {:?}", byte),
        }
        // Add SVG command for updated turtle position
        path_data.push(Command::Line(
            Position::Absolute,
            (turtle.x, turtle.y).into(),
        ));
        // Ensure turtle stays within canvas bounds
        turtle.wrap();
    }
    path_data
}

/// Generate an SVG `Document` from a sequence of SVG `Command`s.
fn generate_svg(path_data: Vec<Command>) -> Document {
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "#ffffff");

    let border = background
        .clone()
        .set("fill-opacity", "0.0")
        .set("stroke", "#cccccc")
        .set("stroke-width", 3 * STROKE_WIDTH);

    let sketch = Path::new()
        .set("fill", "none")
        .set("stroke", "#2f2f2f")
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-opacity", "0.9")
        .set("d", Data::from(path_data));

    let document = Document::new()
        .set("viewBox", (0, 0, HEIGHT, WIDTH))
        .set("height", HEIGHT)
        .set("width", WIDTH)
        .set("style", "style=\"outline: 5px solid #800000;\"")
        .add(background)
        .add(sketch)
        .add(border);

    document
}

fn main() {
    // Parse CLI args
    let args = env::args().collect::<Vec<String>>();
    let input = args.get(1).unwrap();
    let default_filename = format!("{}.svg", input);
    let save_to = args.get(2).unwrap_or(&default_filename);

    // SVG generation pipeline
    let operations = parse(input);
    let path_data = convert(&operations);
    let document = generate_svg(path_data);
    svg::save(save_to, &document).unwrap();
}
