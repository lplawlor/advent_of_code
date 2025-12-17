use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    iter::Iterator,
    ops::Index,
};

const EXAMPLE: bool = false;
const INPUT_PATH: &str = if EXAMPLE { "example" } else { "input" };

/// A generic struct for storing unique keys with non-negative counters
struct Counters<T>(HashMap<T, usize>);

impl<T: Copy + Eq + Hash> Counters<T> {
    /// Returns True if there are no counters associated with any keys
    ///
    /// Wrapper for HashMap::is_empty
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Set the value of the counter associated with the key
    ///
    /// Returns Some(old_value) if the key already had a counter.
    ///
    /// Wrapper for HashMap::insert
    fn set(&mut self, key: &T, value: usize) -> Option<usize> {
        self.0.insert(*key, value)
    }

    /// Get the value of the counter associated with the key
    ///
    /// Wrapper for HashMap::get, except clones the inner value
    fn get(&self, key: &T) -> Option<usize> {
        self.0.get(key).cloned()
    }

    /// If some counter has the given value, remove it and return the key
    ///
    /// If mutliple counters are present with the value, only one key is removed and returned.
    /// Which one is removed is not guaranteed.
    fn pop_equal(&mut self, value: usize) -> Option<T> {
        let possible_match = self
            .0
            .iter()
            .find(|(_, count)| **count == value)
            .map(|(key, _)| *key);

        if let Some(key) = possible_match {
            self.0.remove(&key);

            Some(key)
        } else {
            None
        }
    }

    /// Increase a counter by the given amount.
    ///
    /// If the key is not present, this method has no effect.
    fn increase(&mut self, key: &T, amount: usize) {
        self.0.entry(*key).and_modify(|counter| *counter += amount);
    }

    /// Decrease a counter by the given amount.
    ///
    /// If the key is not present, this method has no effect.
    fn decrease(&mut self, key: &T, amount: usize) {
        self.0.entry(*key).and_modify(|counter| *counter -= amount);
    }

    /// Create a new Counters object from an iterator of keys,
    /// with each counter initialized from each key using the given function.
    fn init_from(keys: impl Iterator<Item = T>, count_func: impl Fn(T) -> usize) -> Self {
        let mut result = Self(HashMap::new());

        for key in keys {
            result.set(&key, count_func(key));
        }

        result
    }
}

/// Allow indexing Counters by key
///
/// Wrapper for HashMap::index
impl<T: Eq + Hash> Index<T> for Counters<T> {
    type Output = usize;

    fn index(&self, index: T) -> &Self::Output {
        self.0.index(&index)
    }
}

/// Struct representing a Simple Unweighted Digraph
struct Digraph<T>(HashMap<T, HashSet<T>>);

impl<T: Eq + Hash + Copy> Digraph<T> {
    // Return an iterator across the vertices of the graph
    fn vertices(&self) -> impl Iterator<Item = T> {
        self.0.keys().cloned()
    }

    // Return an iterator across all vertices connected to the given vertex via outgoing edges
    fn outgoing_vertices(&self, vertex: &T) -> impl Iterator<Item = T> {
        self.0.get(vertex).into_iter().flatten().cloned()
    }

    /// Return an iterator across all vertices connected to the given vertex via incoming edges
    fn incoming_vertices(&self, vertex: &T) -> impl Iterator<Item = T> {
        self.0
            .iter()
            .filter_map(|(other_vertex, other_vertex_destinations)| {
                if other_vertex_destinations.contains(vertex) {
                    Some(other_vertex)
                } else {
                    None
                }
            })
            .cloned()
    }

    /// Inserts a directed edge between the given vertices.
    ///
    /// This method can add an edge which creates a cycle in the graph.
    fn insert_edge(&mut self, from: T, to: T) -> bool {
        self.0.entry(to).or_default();
        self.0.entry(from).or_default().insert(to)
    }

    /// Count the number of paths from the start vertex to all vertices in the graph.
    ///
    /// This method will panic if the graph is not acyclic.
    fn count_paths(&self, start: T) -> Counters<T> {
        // Store counts of the number of incoming edge for each vertex in the graph.
        // We will imagine removing vertices later,
        // so we can update these counts without *actually* modifying the graph.
        let mut incoming_counters = Counters::init_from(self.vertices(), |vertex| {
            self.incoming_vertices(&vertex).count()
        });

        // Store counts of the number of paths leading from start to each vertex.
        // Initally, all vertices are assumed to have 0 such paths,
        // except for start itself, which has 1.
        let mut path_counters =
            Counters::init_from(
                self.vertices(),
                |server| if server == start { 1 } else { 0 },
            );

        // Repeatedly remove vertices with no incoming edges (in-degree 0) until none are left.
        while let Some(vertex) = incoming_counters.pop_equal(0) {
            let paths_to_vertex = path_counters.get(&vertex).expect(
                "path_counters should be initialized with counters for all of the vertices",
            );

            // For each vertex connected to the removed one by an outgoing edge
            for next_vertex in self.outgoing_vertices(&vertex) {
                // Decrement the count of incoming edges
                incoming_counters.decrease(&next_vertex, 1);

                // Increase the number of paths reaching the next vertex
                // by the number of paths reaching the removed vertex
                path_counters.increase(&next_vertex, paths_to_vertex);
            }
        }

        // Removing each vertex with in-degree 0 until none are left will always
        // yield an empty graph if the original graph was acyclic.
        if !incoming_counters.is_empty() {
            panic!("Network should be acyclic!");
        }

        path_counters
    }
}

/// Creat a new network as a Digraph from the given file
fn parse_network_from_file(file: &str) -> Result<Digraph<&str>, &'static str> {
    let mut result = Digraph(HashMap::new());

    for line in file.lines() {
        let (from, tos) = line.split_once(": ").ok_or("Missing colon and space!")?;

        for to in tos.split(" ") {
            result.insert_edge(from, to);
        }
    }

    Ok(result)
}

fn main() {
    let file = std::fs::read_to_string(INPUT_PATH)
        .expect("INPUT_PATH should contain the path of the input file");

    let network = parse_network_from_file(&file).expect("File should be parseable as a Digraph");

    let paths_from_you = network.count_paths("you");
    let paths_from_svr = network.count_paths("svr");
    let paths_from_fft = network.count_paths("fft");
    let paths_from_dac = network.count_paths("dac");

    let you_out = paths_from_you["out"];

    let svr_dac = paths_from_svr["dac"];
    let svr_fft = paths_from_svr["fft"];

    let fft_dac = paths_from_fft["dac"];
    let fft_out = paths_from_fft["out"];

    let dac_fft = paths_from_dac["fft"];
    let dac_out = paths_from_dac["out"];

    // The product of the number of paths along each segment forms the number of total paths.
    // There are two options for the path from svr to out which passes through dac and fft.
    // One will be impossible (product of 0) and one will be possible.
    let svr_dac_fft_out = svr_dac * dac_fft * fft_out;
    let svr_fft_dac_out = svr_fft * fft_dac * dac_out;

    println!("you -> out: {you_out} paths");
    println!("svr -> dac -> fft -> out: {svr_dac_fft_out} paths ");
    println!("svr -> fft -> dac -> out: {svr_fft_dac_out} paths ");
}
