use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

const EXAMPLE: bool = false;
const INPUT_PATH: &str = if EXAMPLE { "example" } else { "input" };
const START_CHAR: char = 'S';
const SPLITTER_CHAR: char = '^';
const BEAM_CHAR: char = '|';
const EMPTY_CHAR: char = '.';

#[derive(Clone, Copy)]
enum QuantumGridTile {
    Splitter,
    Empty,
    Superposition(u64),
}
use QuantumGridTile::{Empty, Splitter, Superposition};

// Display a QuantumGridTile, using a single character for each possibility.
//
// The value attached to any Superposition is not displayed.
impl Display for QuantumGridTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Determine the corrresponding char, then use the char implementation of Display
        match self {
            Empty => EMPTY_CHAR,
            Superposition(_) => BEAM_CHAR,
            Splitter => SPLITTER_CHAR,
        }
        .fmt(f)
    }
}

impl TryFrom<char> for QuantumGridTile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            EMPTY_CHAR => Ok(Empty),
            START_CHAR => Ok(Superposition(1)),
            SPLITTER_CHAR => Ok(Splitter),
            _ => Err("Only .S^ are supported!"),
        }
    }
}

// Add together two QuantumGridTiles
//
// The sum of two QuantumGridTiles is not necessarily defined,
// as a Splitter cannot be added.
//
// Superposition(a) + Superposition(b) becomes Superposition(a+b).
// Empty is treated as Superposition(0).
impl Add for QuantumGridTile {
    type Output = Result<Self, &'static str>;

    fn add(self, rhs: Self) -> Self::Output {
        let error = Err("Cannot add a Splitter!");

        match self {
            Splitter => error,
            Empty => match rhs {
                Splitter => error,
                _ => Ok(rhs),
            },
            Superposition(lhs_value) => match rhs {
                Splitter => error,
                Empty => Ok(self),
                Superposition(rhs_value) => Ok(Superposition(lhs_value + rhs_value)),
            },
        }
    }
}

/// Add two QuantumGridTiles and assign the result to the former.
///
/// Panics if one of the tiles is a Splitter
impl AddAssign for QuantumGridTile {
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self + rhs).unwrap();
    }
}

fn main() {
    let file = std::fs::read_to_string(INPUT_PATH)
        .expect("INPUT_PATH should contain the path of the input file");

    // Convert the file into a 2D vector of QuantumGridTiles, using the TryFrom implementation
    let mut quantum_grid: Vec<Vec<QuantumGridTile>> = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|value| QuantumGridTile::try_from(value).unwrap())
                .collect()
        })
        .collect();

    let grid_height = quantum_grid.len();
    let grid_width = quantum_grid[0].len();

    let mut splits = 0;

    // For every row except the last
    for row in 0..grid_height - 1 {
        for col in 0..grid_width {
            let here = quantum_grid[row][col];

            // Only need to modify the grid when a tachyon beam (Superposition) is encountered
            if matches!(here, Superposition(_)) {
                if matches!(quantum_grid[row + 1][col], Splitter) {
                    // When a Splitter is encountered, the entire superposition here doubles
                    // and splits in two, going left and right in the row below
                    quantum_grid[row + 1][col - 1] += here;
                    quantum_grid[row + 1][col + 1] += here;

                    // Keep track of how many times the beams split
                    splits += 1;
                } else {
                    // When no splitter is encountered, the beam simply travels down
                    quantum_grid[row + 1][col] += here;
                }
            }
        }
    }

    // Add up all the Superposition values in the final row of the grid
    let parallel_universes = quantum_grid
        .last()
        .expect("Should be a last row of the grid")
        .iter()
        .filter_map(|tile| match tile {
            Superposition(value) => Some(value),
            _ => None,
        })
        .sum::<u64>();

    for row in quantum_grid {
        for value in row {
            print!("{value}");
        }
        println!();
    }

    println!("There beam splits {splits} times, forming {parallel_universes} parallel universes.");
}
