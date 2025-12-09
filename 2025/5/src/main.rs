use std::ops::RangeInclusive;

const EXAMPLE: bool = false;
const INPUT_PATH: &str = if EXAMPLE { "example" } else { "input" };

/// Create a Vec of RangeInclusive structs from the range section of the file
fn parse_ranges(range_section: &str) -> Vec<RangeInclusive<u64>> {
    // Each range is on a separate line
    range_section
        .trim()
        .trim()
        .split("\n")
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

/// Create a Vec of ingredient IDs from the ID section of the file
fn parse_ingredients(ingredient_section: &str) -> Vec<u64> {
    // Each ingredient is on a separate line
    ingredient_section
        .trim()
        .split("\n")
        .map(|id_str| {
            id_str
                .parse()
                .expect("Each ingredient ID should be parseable as a u64")
        })
        .collect()
}

/// Combine the two ranges given into one if they overlap
///
/// If the two ranges do not overlap, None is returned
fn range_union(
    range1: &RangeInclusive<u64>,
    range2: &RangeInclusive<u64>,
) -> Option<RangeInclusive<u64>> {
    // Sort the ranges by their starts
    let (first_range, second_range) = if range1.start() < range2.start() {
        (range1, range2)
    } else {
        (range2, range1)
    };

    if first_range.contains(second_range.end()) {
        // If the first range contains the second range entirely, clone the first range.
        Some(first_range.clone())
    } else if second_range.contains(first_range.end())
        || first_range.end() + 1 == *second_range.start()
    {
        // If the ranges overlap partially or touch at their ends,
        // make a new range from the start of the first to the end of the second.
        Some(*first_range.start()..=*second_range.end())
    } else {
        None
    }
}

/// Combine all overlapping or touching ranges given, and return the resulting ranges.
/// The returned ranges will be all be separated by 1 or more.
fn simplify_ranges(ranges: &[RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return vec![];
    }

    // Sort the ranges by their endings, such that the ranges which end last come last
    // We sort in this way because we will later be popping from the right of the Vec.
    // If we sorted by start instead of end, some overlapping ranges would not get combined later
    let mut ranges_sorted = Vec::from(ranges);
    ranges_sorted.sort_by_key(|range| *range.end());

    let mut ranges_simplified = vec![];

    // We will pull out one range at a time to work on
    let mut working_range = ranges_sorted
        .pop()
        .expect("ranges_sorted should not be empty");

    // Keep pulling out ranges until the sorted Vec is empty
    while let Some(next_range) = ranges_sorted.pop() {
        if let Some(overlap) = range_union(&working_range, &next_range) {
            // If the next range overlaps with the one we are working on,
            // replace the working range with its union with the next range
            working_range = overlap;
        } else {
            // If they do not overlap, we are done with the working range,
            // so we can move on to working on the next range.
            // We can be certain that there are no further overlaps because
            // of the sorting order used earlier.
            ranges_simplified.push(working_range);
            working_range = next_range;
        }
    }

    ranges_simplified.push(working_range);

    ranges_simplified
}

fn main() {
    let file = std::fs::read_to_string(INPUT_PATH)
        .expect("INPUT_PATH should contain the path of the input file");

    let (range_section, ingredient_section) = file
        .split_once("\n\n")
        .expect("There should be a blank line separating the sections");

    let ranges = parse_ranges(range_section);
    let ingredients = parse_ingredients(ingredient_section);

    let fresh_ingredients = ingredients
        .iter()
        .filter(|ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count();

    println!("There are {fresh_ingredients} fresh ingredients on-hand.");

    // Add up the lengths of all the ranges after combining them together to eliminate overlaps
    let possible_fresh_ingredients = simplify_ranges(&ranges)
        .iter()
        // RangeInclusive doesn't have a len() method, so we must calculate it manually for each range
        .map(|range| range.end() - range.start() + 1)
        .sum::<u64>();

    println!("There are {possible_fresh_ingredients} possible fresh ingredients.");
}
