const EXAMPLE: bool = false;
const INPUT_PATH: &str = if EXAMPLE { "example" } else { "input" };
const PAPER_CHAR: char = '@';

enum GridTile {
    Empty,
    Paper,
}

struct Grid(Vec<Vec<GridTile>>);

impl Grid {
    fn from_file(file: &str) -> Grid {
        let mut grid = vec![];

        for line in file.lines() {
            let mut row_vec = vec![];

            for character in line.chars() {
                if character == PAPER_CHAR {
                    row_vec.push(GridTile::Paper);
                } else {
                    row_vec.push(GridTile::Empty);
                }
            }

            grid.push(row_vec);
        }

        Grid(grid)
    }

    // Return true if and only if the grid contains a GridTile::Paper at the given row and col
    // Return false if the grid contains GridTile::Empty, or if the row and/or col are out of bounds
    fn is_paper_at(&self, row: isize, col: isize) -> bool {
        if row < 0 || col < 0 {
            false
        } else if let Some(row_vec) = self.0.get(row as usize)
            && let Some(value) = row_vec.get(col as usize)
        {
            matches!(value, GridTile::Paper)
        } else {
            false
        }
    }

    // Return true if and only if the given row and column has fewer than 4 paper neighbours,
    // among the 8 possible neighbouring positions.
    fn is_accessible(&self, row: usize, col: usize) -> bool {
        let mut paper_neighbours = 0u8;

        for row_offset in [-1, 0, 1] {
            for col_offset in [-1, 0, 1] {
                // Skip checking the position itself (a tile is not considered its own neighbour)
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                if self.is_paper_at(row as isize + row_offset, col as isize + col_offset) {
                    paper_neighbours += 1;
                }
            }
        }

        paper_neighbours < 4
    }

    // Set each *currently* accessible position in the grid to GridTile::Empty.
    // The tiles are only removed at the end, so removal of earlier tiles will not affect the
    // removal of later tiles.
    fn remove_acccessible(&mut self) {
        let mut accessible_positions = vec![];

        for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                if matches!(self.0[row][col], GridTile::Paper) && self.is_accessible(row, col) {
                    accessible_positions.push((row, col));
                }
            }
        }

        for (row, col) in accessible_positions {
            self.0[row][col] = GridTile::Empty;
        }
    }

    // Count the number of GridTile::Paper on the grid
    fn count_paper(&self) -> usize {
        self.0
            .iter()
            .flatten()
            .filter(|tile| matches!(tile, GridTile::Paper))
            .count()
    }
}

fn main() {
    let file = std::fs::read_to_string(INPUT_PATH)
        .expect("INPUT_PATH should contain the path of the input file");

    let mut grid = Grid::from_file(&file);

    let mut paper_count = grid.count_paper();

    let mut total_removed = 0;

    // This value of 1 is meaningless, it just needs to be primed to != 0 before the loop
    let mut just_removed = 1;

    // Repeatedly remove the accessible paper until no more are accesssible.
    while just_removed != 0 {
        grid.remove_acccessible();

        let new_paper_count = grid.count_paper();

        just_removed = paper_count - new_paper_count;

        // Print how many were removed at each step
        println!("Removed {} accesssible paper rolls.", just_removed);

        // Count the total number removed
        total_removed += just_removed;

        paper_count = new_paper_count;
    }

    println!("Removed {} paper rolls in total.", total_removed);
}
