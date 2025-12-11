use std::fmt::Display;

const EXAMPLE: bool = false;
const INPUT_PATH: &str = if EXAMPLE { "example" } else { "input" };
const OPERAND_FORMAT_WIDTH: usize = 4;
const ANSWER_FORMAT_WIDTH: usize = 15;

/// The valid operators in Cephalopod Math
#[derive(Copy, Clone)]
enum Operator {
    Multiply,
    Add,
}

impl TryFrom<&str> for Operator {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "*" => Ok(Operator::Multiply),
            "+" => Ok(Operator::Add),
            _ => Err("Only * and + are supported operators!"),
        }
    }
}

/// A Cephalopod Math problem is a set of integer operands linked by a single commutative operator
struct Problem {
    operands: Vec<u64>,
    operator: Operator,
}

impl Problem {
    /// Create a new problem with the given operator, but an empty list of operandd
    fn new(operator: Operator) -> Problem {
        Problem {
            operands: vec![],
            operator,
        }
    }

    /// Add the given operand to the problem
    fn push_operand(&mut self, operand: u64) {
        self.operands.push(operand);
    }

    /// Evaluate the expression by repeatedly applying the operator to the operands
    ///
    /// If the expression has no operands, the identity element of the operator is returned.
    /// I.e. 0 for addition, 1 for multiplication
    fn evaluate(&self) -> u64 {
        // Get the u64 method and identity element corresponding to the operator
        let (operation, mut accumulator): (fn(u64, u64) -> u64, u64) = match self.operator {
            Operator::Multiply => (u64::strict_mul, 1),
            Operator::Add => (u64::strict_add, 0),
        };

        // Repeatedly apply the operator on the operands
        for operand in &self.operands {
            accumulator = operation(accumulator, *operand);
        }

        accumulator
    }

    fn parse_from_file(file: &str, naive: bool) -> Vec<Problem> {
        // This vector will form the return value
        let mut problems = vec![];

        // Convert the file to a 2D vector of characters, where each row is a line of the file
        let file_grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
        let grid_height = file_grid.len();
        let grid_width = file_grid[0].len();

        // Get the index of each column containing only spaces
        let mut empty_cols: Vec<usize> = (0..grid_width)
            .filter(|&col| file_grid.iter().all(|row| row[col] == ' '))
            .collect();

        // The final problem is not proceeded by a column of spaces, but we can pretend
        empty_cols.push(grid_width);

        // The start_col and end_col will become the lower (inclusive) and
        // upper (exclusive) index bounds of the each problem in the file
        let mut start_col = 0;
        for end_col in empty_cols {
            // The last line of the file contains the operators
            let operator_string: String = file_grid[grid_height - 1][start_col..end_col]
                .iter()
                .collect();

            // Trim the operator string, convert it to an Operator enum,
            // and create a new empty problem with that operator.
            let mut problem = Problem::new(Operator::try_from(operator_string.trim()).unwrap());

            if naive {
                // In the naive approach, we read the operands from left-to-right between the column bounds
                // Note that we are skipping the last row, as it has the operator, not the operand
                for file_row in file_grid.iter().take(grid_height - 1) {
                    let operand_string: String = file_row[start_col..end_col].iter().collect();

                    problem.push_operand(operand_string.trim().parse().unwrap());
                }
            } else {
                // In the correct approach, we read the operands from top-to-bottom between the column bounds
                for col in start_col..end_col {
                    let mut operand_string = String::new();

                    for file_row in file_grid.iter().take(grid_height - 1) {
                        operand_string.push(file_row[col]);
                    }

                    problem.push_operand(operand_string.trim().parse().unwrap());
                }
            }

            problems.push(problem);

            // The next problem starts just after the empty column
            start_col = end_col + 1;
        }

        problems
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operator_char = match self.operator {
            Operator::Multiply => '*',
            Operator::Add => '+',
        };

        let mut operands_iter = self.operands.iter();

        let formatter_width = f.width();

        // The first operand is treated specially, as it is not preceded by the operator
        if let Some(first_operand) = operands_iter.next() {
            // Handle when the width format specifier is provided, and when it is not
            if let Some(width) = formatter_width {
                write!(f, "{first_operand:width$}")?;
            } else {
                write!(f, "{first_operand}")?;
            };

            // The remaining operators are formatted the same, except precedeed by the operator
            for operand in operands_iter {
                write!(f, " {operator_char} ")?;

                if let Some(width) = formatter_width {
                    write!(f, "{operand:width$}")?;
                } else {
                    write!(f, "{operand}")?;
                };
            }
        }

        Ok(())
    }
}

fn main() {
    let file = std::fs::read_to_string(INPUT_PATH)
        .expect("INPUT_PATH should contain the path of the input file");

    for (title, problems) in [
        ("Naive Approach:", Problem::parse_from_file(&file, true)),
        ("Correct Approach:", Problem::parse_from_file(&file, false)),
    ] {
        println!("{title}");

        let mut grand_total = 0;

        for problem in &problems {
            let answer = problem.evaluate();
            grand_total += answer;

            println!("{answer:ANSWER_FORMAT_WIDTH$} = {problem:OPERAND_FORMAT_WIDTH$}");
        }

        println!("{} +", "-".repeat(ANSWER_FORMAT_WIDTH));
        println!("{grand_total:ANSWER_FORMAT_WIDTH$}");
        println!();
    }
}
