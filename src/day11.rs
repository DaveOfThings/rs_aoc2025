use std::collections::HashMap;

use crate::day::{Day, Answer};

// A representation of the puzzle inputs.
struct Input {
    connections: Vec<(String, Vec<String>)>
}

impl Input {
    fn read(text: &str) -> Input 
    {
        let mut connections = Vec::new();
        for line in text.lines() {
            // Process one line
            let parts: Vec<&str> = line.split(": ").collect();
            assert_eq!(parts.len(), 2);
            let name = parts[0].to_string();
            let connected: Vec<String> = parts[1].split(" ").map(|s| {s.to_string()}).collect();

            connections.push((name, connected));
        }

        Input { connections }
    }
}

struct Node {
    id: usize,
    outputs: Vec<usize>,
}

impl Node {
    pub fn new(id: usize, outputs: Vec<usize>) -> Node {
        Node { id, outputs }
    }
}

struct Reactor {
    node_ids: HashMap<String, usize>,
    inputs_to: Vec<Vec<usize>>,
}

impl Reactor {
    pub fn new(input: &Input) -> Reactor {
        // Assign a node id to each node by name
        let mut next_id = 0;
        let mut node_ids: HashMap<String, usize> = HashMap::new();

        // Create a node for "out".  It doesn't appear as a named node in connections.
        node_ids.insert("out".to_string(), next_id);
        next_id += 1;

        // Assign ids to remaining nodes
        input.connections.iter()
            .for_each(|conn| {
                node_ids.insert(conn.0.clone(), next_id);
                next_id += 1;
            });

        // Create a vector of Nodes, one for each node id.
        let mut nodes = Vec::new();
        nodes.push(Node::new(0, Vec::new()));

        input.connections.iter()
            .for_each(|(node_name, output_names)| {
                let id = node_ids[node_name];
                let outputs = output_names.iter()
                    .map(|name| {
                        node_ids[name]
                    })
                    .collect();
                
                nodes.push(Node::new(id, outputs));
            });

        // Compute the inputs to each node
        // Start with an empty Vec of Vec structure.
        let mut inputs_to = Vec::new();
        nodes.iter().for_each(|_node| {
            inputs_to.push(Vec::new());
        });
        nodes.iter().for_each(|node_a| {
            node_a.outputs.iter().for_each(|node_b| {
                inputs_to[*node_b].push(node_a.id);
            })
        });

        Reactor { node_ids, inputs_to }
    }

    fn num_paths_to(&self, from: usize, to: usize, paths_to_cache: &mut HashMap<usize, usize>) -> usize {
        let num = if to == from {
            1
            // println!("Paths to {to} : {num} (degenerate case)");
        }
        else if paths_to_cache.contains_key(&to) {
            paths_to_cache[&to]
            // println!("Paths to {to} : {num} (from cache)");
        }
        else {
            let num = self.inputs_to[to].iter().map(|&in_node| {
                      self.num_paths_to(from, in_node, paths_to_cache)
                  })
                  .sum();

            // Add to the cache.
            paths_to_cache.insert(to, num);

            num

            // println!("Paths to {to} : {num} (added to cache)");
        };

        num
    }

    pub fn num_paths_p1(&self) -> usize {
        let mut paths_to_cache: HashMap<usize, usize> = HashMap::new();

        self.num_paths_to(self.node_ids["you"], self.node_ids["out"], &mut paths_to_cache)
    }

    pub fn num_paths_between(&self, from: &str, to: &str) -> usize {
        let mut paths_to_cache: HashMap<usize, usize> = HashMap::new();

        self.num_paths_to(self.node_ids[from], self.node_ids[to], &mut paths_to_cache) 
    }
}

pub struct Day11 {
}

impl Day11 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl Day for Day11 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        // Read input file into Input struct
        let input = Input::read(text);
        let reactor = Reactor::new(&input);

        Answer::Numeric(reactor.num_paths_p1())
    }

    fn part2(&self, text: &str) -> Answer {

        // Read input file into Input struct
        let input = Input::read(text);
        let reactor = Reactor::new(&input);

        let svr_dac = reactor.num_paths_between("svr", "dac");
        let dac_fft = reactor.num_paths_between("dac", "fft");
        let fft_out = reactor.num_paths_between("fft", "out");

        let svr_fft = reactor.num_paths_between("svr", "fft");
        let fft_dac = reactor.num_paths_between("fft", "dac");
        let dac_out = reactor.num_paths_between("dac", "out");

        let paths = svr_dac * dac_fft * fft_out +
                    svr_fft * fft_dac * dac_out;

        Answer::Numeric(paths)
    }
}

#[cfg(test)]
mod test {
    use crate::day11::{Day11, Input, Reactor};
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const EXAMPLE2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.connections.len(), 10);
        assert_eq!(input.connections[0].0, "aaa");
        assert_eq!(input.connections[0].1, vec!["you", "hhh"]);
        assert_eq!(input.connections[9].0, "iii");
        assert_eq!(input.connections[9].1, vec!["out"]);
    }

    #[test]
    fn test_read_part2() {
        let input = Input::read(EXAMPLE2);

        assert_eq!(input.connections.len(), 13);
        assert_eq!(input.connections[0].0, "svr");
        assert_eq!(input.connections[0].1, vec!["aaa", "bbb"]);
        assert_eq!(input.connections[9].0, "dac");
        assert_eq!(input.connections[9].1, vec!["fff"]);
    }

    #[test]
    fn test_create_reactor() {
        let input = Input::read(EXAMPLE1);
        let reactor = Reactor::new(&input);

        assert_eq!(reactor.inputs_to.len(), 11);
        assert_eq!(reactor.node_ids.len(), 11);
    }

    
    #[test]
    fn test_num_paths_p1() {
        let input = Input::read(EXAMPLE1);
        let reactor = Reactor::new(&input);

        assert_eq!(reactor.num_paths_p1(), 5);
    }

    #[test]
    fn test_num_paths_p2() {
        let input = Input::read(EXAMPLE2);
        let reactor = Reactor::new(&input);

        let svr_dac = reactor.num_paths_between("svr", "dac");
        let dac_fft = reactor.num_paths_between("dac", "fft");
        let fft_out = reactor.num_paths_between("fft", "out");

        let svr_fft = reactor.num_paths_between("svr", "fft");
        let fft_dac = reactor.num_paths_between("fft", "dac");
        let dac_out = reactor.num_paths_between("dac", "out");

        let paths = svr_dac * dac_fft * fft_out +
                    svr_fft * fft_dac * dac_out;
        assert_eq!(paths, 2);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d = Day11::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(5));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day11::new();
        assert_eq!(d.part2(EXAMPLE2), Answer::Numeric(2));
    }
    
}
