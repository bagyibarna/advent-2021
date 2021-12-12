use std::collections::{hash_map::Entry, HashMap};

const MAX_SIZE: usize = 16;

#[derive(Default, Debug)]
struct Node<'a> {
    len: usize,
    name: &'a str,
    multi_entry: bool,
    connections: [u8; MAX_SIZE],
}

#[derive(Default, Debug)]
struct Graph<'a> {
    table: [Node<'a>; MAX_SIZE],
    names: HashMap<&'a str, usize>,
    start: usize,
    end: usize,
}

impl<'a> Graph<'a> {
    fn ensure_node(&mut self, name: &'a str) -> usize {
        let new_num = self.names.len();

        match self.names.entry(name) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                entry.insert(new_num);
                self.table[new_num].multi_entry = (name.as_bytes()[0] as char).is_ascii_uppercase();
                self.table[new_num].name = name;
                new_num
            }
        }
    }

    fn add_connection(&mut self, from: &'a str, to: &'a str) {
        let from_i = self.ensure_node(from);
        let to_i = self.ensure_node(to);

        let from_connection = &mut self.table[from_i];
        from_connection.connections[from_connection.len] = to_i as u8;
        from_connection.len += 1;

        let to_connection = &mut self.table[to_i];
        to_connection.connections[to_connection.len] = from_i as u8;
        to_connection.len += 1;
    }

    fn finalize(mut self) -> Self {
        self.start = self.names["start"];
        self.end = self.names["end"];

        self
    }

    fn backtrack_visit(
        &self,
        path: &'a mut Vec<u8>,
        visits: &mut [u8; MAX_SIZE],
        doubled: bool,
    ) -> (usize, usize) {
        let curr = *path.last().unwrap();
        let curr_row = &self.table[curr as usize];
        let mut result = (0, 0);
        for next_ind in 0..curr_row.len {
            let next_node = curr_row.connections[next_ind] as usize;
            path.push(next_node as u8);

            let needs_doubling = !self.table[next_node].multi_entry && visits[next_node] > 0;
            if needs_doubling && (doubled || next_node == self.start) {
                path.pop();
                continue;
            }

            if next_node == self.end {
                result.1 += 1;
                if !doubled {
                    result.0 += 1;
                }
            } else {
                visits[next_node] += 1;

                let child_result = self.backtrack_visit(path, visits, doubled || needs_doubling);
                result.0 += child_result.0;
                result.1 += child_result.1;

                visits[next_node] -= 1;
            }

            path.pop();
        }

        result
    }
}

fn solve(graph: &Graph) {
    let mut path = vec![graph.start as u8];
    let mut visits = [0; MAX_SIZE];
    visits[graph.start] += 1;
    let result = graph.backtrack_visit(&mut path, &mut visits, false);
    println!("result: {:?}", result);
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let graph = content
        .lines()
        .filter_map(|line| line.split_once('-'))
        .fold(Graph::default(), |mut graph, (from, to)| {
            graph.add_connection(from, to);
            graph
        })
        .finalize();

    solve(&graph);
}
