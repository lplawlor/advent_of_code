const EXAMPLE: bool = false;
const INPUT_PATH: &str = if EXAMPLE { "example" } else { "input" };

fn parse_banks(file: &str) -> Vec<Vec<u128>> {
    file.lines()
        .map(|bank_str| {
            bank_str
                .chars()
                .map(|battery_str| {
                    battery_str
                        .to_digit(10)
                        .expect("Each character should be a digit") as u128
                })
                .collect()
        })
        .collect()
}

/// Return the maximum joltage of a bank, limited to the specified number of digits
///
/// A joltage is formed from a bank by concatenating several digits
/// in the same order they appear in the bank.
/// The digits do not need to be adjacent in the bank to be concatenated for a joltage.
fn max_joltage(bank: &[u128], digits: usize) -> u128 {
    if digits == 0 {
        return 0;
    }

    // We need to be able to get (digits - 1) more digits after the first one,
    // so we only search the beginning of the bank slice, leaving enough at the end.
    let first_digit = bank[..bank.len() - (digits - 1)]
        .iter()
        .max()
        .expect("Should be a maximum value");

    // Find out what index that maximum came from
    let first_digit_index = bank
        .iter()
        .position(|digit| digit == first_digit)
        .expect("first_digit should be in the bank");

    // For the rest of the voltage, our search space is limited to after the first digit
    // We can recursively find the maximum voltage for this smaller bank slice with 1 less digit.
    let remaining_digits = max_joltage(&bank[first_digit_index + 1..], digits - 1);

    // Concatenate the first digit and the remaining digits
    first_digit * 10u128.pow(digits as u32 - 1) + remaining_digits
}

fn main() {
    let file = std::fs::read_to_string(INPUT_PATH)
        .expect("INPUT_PATH should contain the path of the input file");

    let banks = parse_banks(&file);

    let mut max_joltages_2 = vec![];
    let mut max_joltages_12 = vec![];

    for bank in banks {
        max_joltages_2.push(max_joltage(&bank, 2));
        max_joltages_12.push(max_joltage(&bank, 12));
    }

    println!(
        "Sum of maximum  2-digit joltages: {}",
        max_joltages_2.into_iter().sum::<u128>()
    );
    println!(
        "Sum of maximum 12-digit joltages: {}",
        max_joltages_12.into_iter().sum::<u128>()
    )
}
