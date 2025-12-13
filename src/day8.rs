use std::{cmp::Ordering, collections::HashSet};

use crate::day::{Day, Answer};
use vector3d::Vector3d;
use regex::Regex;

// A representation of the puzzle inputs.
struct Input {
    coords: Vec<Vector3d<usize>>,
}

impl Input {
    fn read(text: &str) -> Input 
    {
        let vector_re = Regex::new("([0-9]+),([0-9]+),([0-9]+)").unwrap();
        let mut coords = Vec::new();

        for line in text.lines() {
            // Process one line
            if let Some(m) = vector_re.captures(line) {
                let x = m[1].parse::<usize>().unwrap();
                let y = m[2].parse::<usize>().unwrap();
                let z = m[3].parse::<usize>().unwrap();

                coords.push(Vector3d::new(x, y, z));
            }
        }

        Input { coords }
    }

    // returns (index1, index2) in vector shorted from smallest distance to largest.
    fn pairs_by_dist(&self) -> Vec<(usize, usize)> {
        let mut pairs = Vec::new();
        for n in 0..self.coords.len()-1 {
            for m in n+1..self.coords.len() {
                pairs.push((n, m));
            }
        }

        pairs.sort_by(|pair_a, pair_b| {
            let a1 = self.coords[pair_a.0];
            let a2 = self.coords[pair_a.1];
            let b1 = self.coords[pair_b.0];
            let b2 = self.coords[pair_b.1];

            let dx = a1.x.abs_diff(a2.x);
            let dy = a1.y.abs_diff(a2.y);
            let dz = a1.z.abs_diff(a2.z);
            let dist_a_squared = dx*dx + dy*dy + dz*dz;

            let dx = b1.x.abs_diff(b2.x);
            let dy = b1.y.abs_diff(b2.y);
            let dz = b1.z.abs_diff(b2.z);
            let dist_b_squared = dx*dx + dy*dy + dz*dz;

            if dist_a_squared < dist_b_squared {
                Ordering::Less
            }
            else if dist_a_squared > dist_b_squared {
                Ordering::Greater
            }
            else {
                Ordering::Equal
            }

        });

        pairs
    }

    fn make_networks(&self, n_connections: usize) -> Vec<HashSet<usize>> {
        let mut networks: Vec<HashSet<usize>> = Vec::new();
        let pairs = self.pairs_by_dist();
        for n in 0..n_connections {
            // Connect pairs[n], adding the resulting network to our Vec<HashSet<>> of
            // networks.
            // If neither element of the pair is in a network, make a new HashSet and
            // add it to the Vec.
            // If one element of the pair is in a network, add the other element to
            // the network (HashSet) the first is in.
            // If both elements are in networks and they are the same network, do nothing.
            // If both elements are in the networks and they are different networks, combine
            // the two networks by doing this:
            //   - Add all of the higher network to the lower one.
            //   - Remove the higher network from the Vec.

            let mut net_0 = None;
            let mut net_1 = None;
            for net_id in 0..networks.len() {
                if networks[net_id].contains(&pairs[n].0) {
                    net_0 = Some(net_id);
                }
                if networks[net_id].contains(&pairs[n].1) {
                    net_1 = Some(net_id);
                }
            }

            if net_0.is_none() && net_1.is_none() {
                // Form a new network with these two nodes
                let mut net = HashSet::new();
                net.insert(pairs[n].0);
                net.insert(pairs[n].1);
                networks.push(net);
            }
            else if let Some(net_0_id) = net_0 && 
                    let Some(net_1_id) = net_1 {
                // Both elements of the pair are already in networks

                if net_0_id == net_1_id {
                    // They are already in the same network, we don't need to do anything.
                }
                else if net_0_id < net_1_id {
                    // Move all of net_1 to net_0
                    let (a, b) = networks.split_at_mut(net_0_id+1);

                    for mover in &b[net_1_id-net_0_id-1] {
                        a[net_0_id].insert(*mover);
                    }
                    networks.remove(net_1_id);
                }
                else {
                    // Move all of net_0 to net_1
                    let (a, b) = networks.split_at_mut(net_1_id+1);

                    for mover in &b[net_0_id-net_1_id-1] {
                        a[net_1_id].insert(*mover);
                    }
                    networks.remove(net_0_id);
                }

            }
            else if let Some(net_1_id) = net_1 {
                // Add component 0 to the network compnent 1 is in.
                networks[net_1_id].insert(pairs[n].0);
            }
            else if let Some(net_0_id) = net_0 {
                // Add component 1 to the network compnent 0 is in.
                networks[net_0_id].insert(pairs[n].1);
            }
        }

        networks.sort_by(|a, b| {
            if a.len() < b.len() {
                Ordering::Less
            }
            else if a.len() > b.len() {
                Ordering::Greater
            }
            else {
                Ordering::Equal
            }
        });
        networks.reverse();

        networks
    }

    // Keep combining boxes until all are in one network.
    // Return the Node ids of the last two combined.
    fn make_one_network(&self) -> Option<(usize, usize)> {
        let mut networks: Vec<HashSet<usize>> = Vec::new();

        // Put each junction box into its own network of one node
        for n in 0..self.coords.len() {
            let mut singular_net = HashSet::new();
            singular_net.insert(n);
            networks.push(singular_net);
        }

        let pairs = self.pairs_by_dist();
        
        for n in 0..pairs.len()  {
            // Connect pairs[n], adding the resulting network to our Vec<HashSet<>> of
            // networks.
            // If neither element of the pair is in a network, make a new HashSet and
            // add it to the Vec.
            // If one element of the pair is in a network, add the other element to
            // the network (HashSet) the first is in.
            // If both elements are in networks and they are the same network, do nothing.
            // If both elements are in the networks and they are different networks, combine
            // the two networks by doing this:
            //   - Add all of the higher network to the lower one.
            //   - Remove the higher network from the Vec.

            let mut net_0 = None;
            let mut net_1 = None;
            for net_id in 0..networks.len() {
                if networks[net_id].contains(&pairs[n].0) {
                    net_0 = Some(net_id);
                }
                if networks[net_id].contains(&pairs[n].1) {
                    net_1 = Some(net_id);
                }
            }

            if net_0.is_none() && net_1.is_none() {
                // Form a new network with these two nodes
                let mut net = HashSet::new();
                net.insert(pairs[n].0);
                net.insert(pairs[n].1);
                networks.push(net);
            }
            else if let Some(net_0_id) = net_0 && 
                    let Some(net_1_id) = net_1 {
                // Both elements of the pair are already in networks

                if net_0_id == net_1_id {
                    // They are already in the same network, we don't need to do anything.
                }
                else if net_0_id < net_1_id {
                    // Move all of net_1 to net_0
                    let (a, b) = networks.split_at_mut(net_0_id+1);

                    for mover in &b[net_1_id-net_0_id-1] {
                        a[net_0_id].insert(*mover);
                    }
                    networks.remove(net_1_id);
                }
                else {
                    // Move all of net_0 to net_1
                    let (a, b) = networks.split_at_mut(net_1_id+1);

                    for mover in &b[net_0_id-net_1_id-1] {
                        a[net_1_id].insert(*mover);
                    }
                    networks.remove(net_0_id);
                }

            }
            else if let Some(net_1_id) = net_1 {
                // Add component 0 to the network compnent 1 is in.
                networks[net_1_id].insert(pairs[n].0);
            }
            else if let Some(net_0_id) = net_0 {
                // Add component 1 to the network compnent 0 is in.
                networks[net_0_id].insert(pairs[n].1);
            }

            // If we are down to one network, we're done
            if networks.len() == 1 {
                return Some(pairs[n]);
            }
        }

        None
    }
}

pub struct Day8 {
    to_connect: usize
}

impl Day8 {
    pub const fn new() -> Self {
        Self { to_connect: 1000 }
    }

    #[cfg(test)]
    fn to_connect(&mut self, n: usize) {
        self.to_connect = n;
    }
}

impl Day for Day8 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        // Read input file into Input struct

        let input = Input::read(text);   
        let nets = input.make_networks(self.to_connect);

        let result = nets[0].len() * nets[1].len() * nets[2].len();

        Answer::Numeric(result)
    }

    fn part2(&self, text: &str) -> Answer {

        // Read input file into Input struct
        let input = Input::read(text);   

        if let Some(nodes) = input.make_one_network() {
            Answer::Numeric(input.coords[nodes.0].x * input.coords[nodes.1].x)
        }
        else {
            Answer::None
        }


    }
}

#[cfg(test)]
mod test {
    use crate::day8::{Day8, Input};
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.coords.len(), 20);
        assert_eq!(input.coords[0].x, 162);
        assert_eq!(input.coords[0].y, 817);
        assert_eq!(input.coords[0].z, 812);
    }

    #[test]
    fn test_ordering() {
        let input = Input::read(EXAMPLE1);   

        let pairs = input.pairs_by_dist();
        assert_eq!(pairs.len(), 20*19/2);
        assert_eq!(pairs[0].0, 0);
        assert_eq!(pairs[0].1, 19);
    }

    #[test]
    fn test_make_networks() {
        let input = Input::read(EXAMPLE1);   

        let nets = input.make_networks(10);
        assert_eq!(nets.len(), 4);
        assert_eq!(nets[0].len(), 5);
        assert_eq!(nets[1].len(), 4);
        assert_eq!(nets[2].len(), 2);
        assert_eq!(nets[3].len(), 2);
    }

    #[test]
    fn test_make_one_network() {
        let input = Input::read(EXAMPLE1);   

        let nodes = input.make_one_network();
        assert_eq!(nodes, Some((10, 12)));
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let mut d = Day8::new();
        d.to_connect(10);

        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(40));
    }


    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day8::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(25272));
    }
    
}
