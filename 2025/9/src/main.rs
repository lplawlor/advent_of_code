use std::cmp::Ordering;

const EXAMPLE: bool = false;
const INPUT_PATH: &str = if EXAMPLE { "example" } else { "input" };

/// GridPosition(x,y) forms the Cartesian coordinates of a single tile in the grid
#[derive(Clone, Copy, PartialEq)]
struct GridPosition(u64, u64);

impl GridPosition {
    /// Given one string of the form "x,y", where x, y are unsigned ints, construct a GridPosition.
    ///
    /// The constructed object will be returned wrapped by Ok.
    /// An Err will be returned if the format is incorrect
    fn parse_one_from_csv(line: &str) -> Result<Self, &'static str> {
        let mut values = line.split(",");

        let x_str = values.next().ok_or("Missing x value!")?;
        let y_str = values.next().ok_or("Missing y value!")?;

        let x = x_str.parse().map_err(|_| "Could not parse x!")?;
        let y = y_str.parse().map_err(|_| "Could not parse y!")?;

        Ok(Self(x, y))
    }

    /// Given a string with each line of the form "x,y", construct a GridPosition from each line.
    ///
    /// The constructed vector of objects will be returned wrapped by Ok.
    /// An Err will be returned if the format was incorrect on any of the lines.
    fn parse_many_from_csv(file: &str) -> Result<Vec<Self>, &'static str> {
        let mut points = vec![];

        for line in file.lines() {
            points.push(Self::parse_one_from_csv(line)?);
        }

        Ok(points)
    }
}

/// A GridLine is a horizontal or vertical line between and including two grid points,
/// or a unit line containing just one grid point.
///
/// For Horizontal lines, points are stored as (leftmost, rightmost).
/// For Vertical lines, points are stored as (lowermost, uppermost).
#[derive(PartialEq)]
enum GridLine {
    Horizontal(GridPosition, GridPosition),
    Vertical(GridPosition, GridPosition),
    Unit(GridPosition),
}

impl GridLine {
    /// Create and return a new GridLine from the given points.
    /// Returns an Err if they are not equal or vertically or horizontally aligned
    fn new(position_a: GridPosition, position_b: GridPosition) -> Result<Self, &'static str> {
        let x_cmp = position_a.0.cmp(&position_b.0);
        let y_cmp = position_a.1.cmp(&position_b.1);

        match x_cmp {
            Ordering::Equal => match y_cmp {
                Ordering::Equal => Ok(Self::Unit(position_a)),
                Ordering::Less => Ok(Self::Vertical(position_a, position_b)),
                Ordering::Greater => Ok(Self::Vertical(position_b, position_a)),
            },
            Ordering::Less => match y_cmp {
                Ordering::Equal => Ok(Self::Horizontal(position_a, position_b)),
                _ => Err("Line cannot be diagonal!"),
            },
            Ordering::Greater => match y_cmp {
                Ordering::Equal => Ok(Self::Horizontal(position_b, position_a)),
                _ => Err("Line cannot be diagonal!"),
            },
        }
    }

    /// Return the length self, inclusive of the end tile(s)
    fn len(&self) -> u64 {
        match self {
            Self::Horizontal(position_a, position_b) => position_b.0 - position_a.0 + 1,
            Self::Vertical(position_a, position_b) => position_b.1 - position_a.1 + 1,
            Self::Unit(_) => 1,
        }
    }

    /// Returns true if self contains all points in other.
    ///
    /// That is, the two lines must be parallel and overlapping,
    /// the end points of other must lie on self.
    ///
    /// Note that if self and other are equal, this method returns true.
    fn is_superline(&self, other: &Self) -> bool {
        match self {
            Self::Unit(_) => false,
            Self::Horizontal(self_left, self_right) => match other {
                Self::Horizontal(other_left, other_right) => {
                    self_left.1 == other_left.1
                        && self_left.0 <= other_left.0
                        && other_right.0 <= self_right.0
                }
                Self::Unit(other_point) => {
                    self_left.1 == other_point.1
                        && self_left.0 <= other_point.0
                        && other_point.0 <= self_right.0
                }
                _ => false,
            },
            Self::Vertical(self_lower, self_upper) => match other {
                Self::Vertical(other_lower, other_upper) => {
                    self_lower.0 == other_lower.0
                        && self_lower.1 <= other_lower.1
                        && other_upper.1 <= self_upper.1
                }
                Self::Unit(other_point) => {
                    self_lower.0 == other_point.0
                        && self_lower.1 <= other_point.1
                        && other_point.1 <= self_upper.1
                }
                _ => false,
            },
        }
    }

    /// Returns true if self is a different line containing all points in other.
    ///
    /// That is, the two lines must be parallel and overlapping,
    /// the end points of other must lie on self,
    /// and other cannot be exactly equal to self.
    fn is_strict_superline(&self, other: &Self) -> bool {
        if self == other {
            false
        } else {
            self.is_superline(other)
        }
    }

    // Returns true if self and other fully cross one another orthogonally.
    //
    // That is, one must be vertical and the other horizontal,
    // and they must intersect at a point which is not one of their end points.
    fn orthogonally_crosses(&self, other: &Self) -> bool {
        match self {
            Self::Horizontal(self_left, self_right) => match other {
                Self::Vertical(other_lower, other_upper) => {
                    self_left != other_lower
                        && self_left != other_upper
                        && self_right != other_lower
                        && self_right != other_upper
                        && self_left.0 < other_lower.0
                        && other_lower.0 < self_right.0
                        && other_lower.1 < self_left.1
                        && self_left.1 < other_upper.1
                }
                _ => false,
            },
            Self::Vertical(self_lower, self_upper) => match other {
                Self::Horizontal(other_left, other_right) => {
                    self_lower != other_left
                        && self_lower != other_right
                        && self_upper != other_left
                        && self_upper != other_right
                        && other_left.0 < self_lower.0
                        && self_lower.0 < other_right.0
                        && self_lower.1 < other_left.1
                        && other_left.1 < self_upper.1
                }
                _ => false,
            },
            Self::Unit(_) => false,
        }
    }
}

/// A Rectangle stores the 4 GridLines which make up its 4 sides
struct Rectangle([GridLine; 4]);

impl Rectangle {
    /// Create a new Rectangle from two of its diagonally opposite corners
    fn new(corner_a: GridPosition, corner_b: GridPosition) -> Self {
        let corner_c = GridPosition(corner_a.0, corner_b.1);
        let corner_d = GridPosition(corner_b.0, corner_a.1);

        Self([
            GridLine::new(corner_a, corner_c).unwrap(),
            GridLine::new(corner_c, corner_b).unwrap(),
            GridLine::new(corner_b, corner_d).unwrap(),
            GridLine::new(corner_d, corner_a).unwrap(),
        ])
    }

    /// Accessor for the 4 sides of the Rectangle
    fn sides(&self) -> &[GridLine; 4] {
        &self.0
    }

    /// Return the 2D area of the rectangle, inclusive of all edge tiles
    fn area(&self) -> u64 {
        // We must add one to the length and width to include both corners
        self.0[0].len() * self.0[1].len()
    }
}

fn main() {
    let file = std::fs::read_to_string(INPUT_PATH)
        .expect("INPUT_PATH should contain the path of the input file");

    let red_tiles = GridPosition::parse_many_from_csv(&file).unwrap();

    // Initialize a list of all unique Rectangles formed with red tiles in two opposite corners,
    // and their areas
    let mut rectangles = vec![];

    // Initialize a list of lines forming the outline of all the red tiles in order
    let mut shape_outline = vec![];

    // We will iterate over pairs of tiles, so skip the first one
    for (i, &tile_a) in red_tiles.iter().enumerate().skip(1) {
        // Each adjacent pair of tiles given in the file forms a line of the outline
        shape_outline.push(
            GridLine::new(tile_a, red_tiles[i - 1])
                .expect("Each tile should be vertically or horizontally aligned to the previous!"),
        );

        // Loop through each possible corner_b to come before corner_a in the list
        // This prevents including the same Rectangle twice
        for &tile_b in &red_tiles[..i] {
            let rectangle = Rectangle::new(tile_a, tile_b);
            let area = rectangle.area();
            rectangles.push((rectangle, area));
        }
    }

    println!(
        "Using red tiles as opposite corners, the maximum rectangular area is {}.",
        rectangles
            .iter()
            .map(|(_, area)| area)
            .max()
            .expect("Should be a maximum area")
    );

    println!(
        "Staying within the outline limits the maximum rectangular area to {}.",
        rectangles
            .iter()
            .filter(|(rectangle, _)| {
                // If the following is true for all 4 sides, the rectangle is contained by the outline:
                rectangle.sides().iter().all(|side| {
                    // If the side does not orthogonally cross or strictly overlap
                    // any part of the outline, it pass outside of the outline.
                    !shape_outline.iter().any(|connective| {
                        side.orthogonally_crosses(connective)
                            || side.is_strict_superline(connective)
                    })
                })
            })
            .map(|(_, area)| area)
            .max()
            .expect("Should be a maximum area")
    );
}
