use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn tile_cost(x: usize, y: usize, matrix: &[Vec<u8>]) -> usize {
    let (addx, x) = (x / matrix.len(), x % matrix.len());
    let (addy, y) = (y / matrix[0].len(), y % matrix[0].len());

    let cost = addx + addy + (matrix[x][y] as usize);
    ((cost - 1) % 9) + 1
}

fn foreach_neighbour(
    x: usize,
    y: usize,
    x_dim: usize,
    y_dim: usize,
    mut f: impl FnMut(usize, usize),
) {
    if x > 0 {
        f(x - 1, y)
    }

    if x < x_dim - 1 {
        f(x + 1, y)
    }

    if y > 0 {
        f(x, y - 1)
    }

    if y < y_dim - 1 {
        f(x, y + 1)
    }
}

fn shortest_path(matrix: &[Vec<u8>], mult: usize) -> Option<usize> {
    let x_dim = matrix.len() * mult;
    let y_dim = matrix[0].len() * mult;
    let goal = (x_dim - 1, y_dim - 1);
    let mut dist = vec![vec![usize::MAX; y_dim]; x_dim];

    let mut queue = BinaryHeap::new();
    dist[0][0] = 0;
    queue.push(State {
        cost: 0,
        x: 0,
        y: 0,
    });

    while let Some(State { x, y, cost }) = queue.pop() {
        if x == goal.0 && y == goal.1 {
            return Some(cost);
        }

        if dist[x][y] < cost {
            continue;
        }

        foreach_neighbour(x, y, x_dim, y_dim, |x, y| {
            let cost = cost + tile_cost(x, y, matrix);
            if cost < dist[x][y] {
                dist[x][y] = cost;
                queue.push(State { cost, x, y });
            }
        });
    }

    None
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let matrix = content
        .lines()
        .map(|line| line.bytes().map(|byte| byte - b'0').collect())
        .collect::<Vec<_>>();

    println!("part1: {}", shortest_path(&matrix, 1).unwrap());
    println!("part2: {}", shortest_path(&matrix, 5).unwrap());
}
