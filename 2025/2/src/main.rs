use std::ops::RangeInclusive;

const EXAMPLE: bool = false;
const INPUT_PATH: &str = if EXAMPLE { "example" } else { "input" };

/// Create a Vec of RangeInclusive structs from a file
fn parse_ranges(file: &str) -> Vec<RangeInclusive<u64>> {
    // Each range is separated by a comma
    file.split(",")
        .map(|range_str| {
            // The start and end of the ranges are separated by a dash
            let (start_str, end_str) = range_str
                .trim()
                .split_once("-")
                .expect("Each range should contain a - between the start and end");

            RangeInclusive::new(
                start_str.parse().expect("start should be parseable as u64"),
                end_str.parse().expect("end should be parseable as u64"),
            )
        })
        .collect()
}

fn main() {
    let file = std::fs::read_to_string(INPUT_PATH)
        .expect("INPUT_PATH should contain the path of the input file");

    let ranges = parse_ranges(&file);

    // Simple invalid IDs are those formed from some sequence repeated twice.
    let mut simple_invalid_ids_sum = 0;

    // All invalid IDs are formed from some sequence repeated any number of times.
    let mut all_invalid_ids_sum = 0;

    for range in ranges {
        for id in range {
            let id_str = id.to_string();
            let id_len = id_str.len();

            // For every possible length of a repeated sequence which could form the ID,
            // starting from the longest possible sequence and getting smaller.
            // If we went from small sequence_len to large instead,
            // we would find that "111111" is 6 "1"s before we find that it is 2 "111"s.
            // So we would miss the fact that it is a simple invalid ID.
            for sequence_len in (1..=id_len / 2).rev() {
                // Ignore sequence lengths which are not factors of the ID length,
                // as they can never be repeated to precisely the ID length.
                if !id_len.is_multiple_of(sequence_len) {
                    continue;
                }

                // Take the first sequence_len slice from the start of the string.
                let sequence = &id_str[0..sequence_len];

                // If each subsequence slice is the same, the ID is a repeated sequence.
                if (sequence_len..id_str.len())
                    .step_by(sequence_len)
                    .all(|index| &id_str[index..index + sequence_len] == sequence)
                {
                    // Therefore, this ID counts as invalid.
                    all_invalid_ids_sum += id;

                    // If the sequence is repeated just twice to form the ID,
                    // it is also a simple invalid ID.
                    if sequence_len * 2 == id_len {
                        simple_invalid_ids_sum += id;
                    }

                    // Once we've found out that the ID is invalid, we can move on.
                    break;
                }
            }
        }
    }

    println!("Sum of simple invalid IDs: {simple_invalid_ids_sum}");
    println!("Sum of all invalid IDs {all_invalid_ids_sum}");
}
