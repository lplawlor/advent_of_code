const EXAMPLE: bool = false;
const INPUT_PATH: &str = if EXAMPLE { "example" } else { "input" };
const DIAL_START: i64 = 50;
const DIAL_SIZE: i64 = 100;

enum Instruction {
    Left(i64),
    Right(i64),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left(amount) => write!(f, "L{amount:03}"),
            Self::Right(amount) => write!(f, "R{amount:03}"),
        }
    }
}

/// Create a Vec of Instruction structs from a file
fn parse_instructions(file: &str) -> Vec<Instruction> {
    let mut instructions = vec![];

    for line in file.lines() {
        // The first char is the direction, the following number is the amount
        let (direction_str, amount_str) = line.split_at(1);

        // Parse the amount as a signed int
        let amount: i64 = amount_str
            .parse()
            .expect("Amount should be parseable as i64");

        // Create the Instruction struct from the line and add it to the Vec
        instructions.push(match direction_str {
            "L" => Instruction::Left(amount),
            "R" => Instruction::Right(amount),
            _ => panic!("First character should be L or R"),
        })
    }

    instructions
}

fn main() {
    let file = std::fs::read_to_string(INPUT_PATH)
        .expect("INPUT_PATH should contain the path of the input file");

    let instructions = parse_instructions(&file);

    let mut dial = DIAL_START;

    let mut visits = 0;
    let mut passes = 0;

    for instruction in instructions {
        print!("{instruction}: {dial:2} ->");

        match instruction {
            Instruction::Left(amount) => {
                // If the dial is already at 0, this should not be counted as passing zero,
                // as it was already counted as a visit.
                // So we, start at DIAL_SIZE instead.
                if dial == 0 {
                    dial += DIAL_SIZE;
                }

                // Turn the dial left by the specified amount.
                dial -= amount;

                // If it goes out of range, continually turn it right by DIAL_SIZE,
                // keeping track of how many full rotations were necessary, as each one passes 0.
                while dial < 0 {
                    dial += DIAL_SIZE;
                    passes += 1;
                }

                // If we end the turn on 0, that counts as a visit to 0.
                if dial == 0 {
                    visits += 1;
                }
            }
            Instruction::Right(amount) => {
                // Turn the dial right by the specified amount.
                dial += amount;

                // If it goes out of range, continually turn it left by DIAL_SIZE,
                // keeping track of how many full rotations were necessary, as each one passes 0.
                while dial > DIAL_SIZE {
                    dial -= DIAL_SIZE;
                    passes += 1;
                }

                // If we are still out of range, the final turn left will leave us at 0.
                // Therefore, ti counts as a visit, not a pass.
                if dial == DIAL_SIZE {
                    dial -= DIAL_SIZE;
                    visits += 1;
                }
            }
        }

        println!(" {dial:2} ({visits:4} visits, {passes:4} passes)");
    }

    println!("Password 1: {visits}");
    println!("Password 2: {}", visits + passes);
}
