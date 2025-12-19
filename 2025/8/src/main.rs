const EXAMPLE: bool = false;
const INPUT_PATH: &str = if EXAMPLE { "example" } else { "input" };
const PART_1_WIRES: usize = if EXAMPLE { 10 } else { 1000 };

/// Cartesian coordinate representation of a point in 3D space
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    /// Given one string of the form "x,y,z", where x, y and z are floats, construct a Point3D
    ///
    /// The constructed Point3D object will be returned wrapped by Ok.
    /// An Err will be returned if the format is incorrect
    fn parse_one_from_csv(line: &str) -> Result<Self, &'static str> {
        let mut values = line.split(",");

        let x_str = values.next().ok_or("Missing x value!")?;
        let y_str = values.next().ok_or("Missing y value!")?;
        let z_str = values.next().ok_or("Missing z value!")?;

        let x = x_str.parse().map_err(|_| "Could not parse x!")?;
        let y = y_str.parse().map_err(|_| "Could not parse y!")?;
        let z = z_str.parse().map_err(|_| "Could not parse z!")?;

        Ok(Self { x, y, z })
    }

    /// Given a string with each line of the form "x,y,z", construct a Point3D from each line
    ///
    /// The constructed vector of Point3D objects will be returned wrapped by Ok.
    /// An Err will be returned if the format was incorrect on any of the lines.
    fn parse_many_from_csv(file: &str) -> Result<Vec<Self>, &'static str> {
        let mut points = vec![];

        for line in file.lines() {
            points.push(Self::parse_one_from_csv(line)?);
        }

        Ok(points)
    }

    /// Calculate the straight-line distance between two points in 3D space using Pythagoras
    fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
}

impl std::fmt::Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x=")?;
        self.x.fmt(f)?;
        write!(f, ", y=")?;
        self.y.fmt(f)?;
        write!(f, ", z=")?;
        self.z.fmt(f)?;
        write!(f, ")")
    }
}

fn main() {
    let file = std::fs::read_to_string(INPUT_PATH)
        .expect("INPUT_PATH should contain the path of the input file");

    let junction_boxes = Point3D::parse_many_from_csv(&file).unwrap();

    // Create a list of all unique pairs of distinct boxes, ignoring order
    // That is, all ways to choose 2 boxes from the list
    let mut box_pairs: Vec<(Point3D, Point3D)> = vec![];

    // box_b will always come before box_a in the list
    // so skip the first value when picking box_a
    for (i, box_a) in junction_boxes.iter().enumerate().skip(1) {
        // Limit the values of box_b to come before box_a in the list
        // This prevents including the same pair in both orders
        for box_b in &junction_boxes[..i] {
            box_pairs.push((*box_a, *box_b));
        }
    }

    // Sort by comparing the distances between pairs of boxes, with the smallest distances first
    // Need to jump through some hoops to sort by f64 in Rust, since f64 does not implement Ord
    box_pairs.sort_unstable_by(|(box_1a, box_1b), (box_2a, box_2b)| {
        box_2a
            .distance(box_2b)
            .partial_cmp(&box_1a.distance(box_1b))
            .expect("The distances should all be comparable")
    });

    // Initialize the list of circuits such that each box is alone in its own circuit
    let mut circuits: Vec<Vec<Point3D>> = junction_boxes
        .iter()
        .map(|&junction_box| vec![junction_box])
        .collect();

    let mut wires = 0;

    // Keep going until all the junction boxes are linked up into one big circuit
    while circuits.len() > 1 {
        // Pop the closest pair of boxes we have not yet checked
        let (box_a, box_b) = box_pairs
            .pop()
            .expect("Should be distances left in the list");

        // Add a wire between box_a and box_b
        wires += 1;

        // We need to index a's circuit instead of getting it directly with .find(),
        // to prevent multiple mutable references being active for circuits
        let mut a_circuit_index = circuits
            .iter_mut()
            .position(|circuit| circuit.contains(&box_a))
            .expect("Some circuit should contain box_a");

        // We only need to merge circuits if box_a and and box_b
        // were not previously in the same circuit
        if !circuits[a_circuit_index].contains(&box_b) {
            let b_circuit_index = circuits
                .iter_mut()
                .position(|circuit| circuit.contains(&box_b))
                .expect("Some circuit should contain box_b");

            // b's circuit is removed
            let mut b_circuit = circuits.remove(b_circuit_index);

            // We must update a's circuit's index if it was affected by b's circuit's removal
            if b_circuit_index < a_circuit_index {
                a_circuit_index -= 1;
            }

            // b's circuit mergees into a's circuit
            circuits[a_circuit_index].append(&mut b_circuit);
        }

        // Problem 1 is concerned with the state of the circuits after adding some number of wires
        if wires == PART_1_WIRES {
            // Find out how many junction boxes are in each circuit and sort in decscending order
            let mut circuit_sizes: Vec<usize> =
                circuits.iter().map(|circuit| circuit.len()).collect();
            circuit_sizes.sort_unstable();
            circuit_sizes.reverse();

            let size_1 = circuit_sizes[0];
            let size_2 = circuit_sizes[1];
            let size_3 = circuit_sizes[2];

            println!(
                "After adding {PART_1_WIRES} wires, the three largest circuits have sizes {size_1}, {size_2} and {size_3}."
            );
            println!(
                "The product of these sizes is {}.",
                size_1 * size_2 * size_3
            );
            println!();
        }

        // Problem 2 is concerned with the last two junction boxes connected to form one big circuit
        if circuits.len() == 1 {
            println!("{box_a} and {box_b} were the last two junction boxes connected.");
            println!(
                "The product of their x coordinates is {}.",
                (box_a.x * box_b.x) as u64
            );
        }

        // Note that the above is an implementation of Kruskal's algorithm
        // for finding the minimum spanning tree of a graph
    }
}
