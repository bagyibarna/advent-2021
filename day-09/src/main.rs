use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let matrix = content.lines().collect::<Vec<_>>();
    let mut low_points = (0..matrix.len()).flat_map(|x| {
        let matrix = &matrix;
        let x = x;
        (0..matrix[x].len()).filter_map(move |y| {
            let val = matrix[x].as_bytes()[y];
            if (x == 0 || matrix[x - 1].as_bytes()[y] > val)
                && (x == matrix.len() - 1 || matrix[x + 1].as_bytes()[y] > val)
                && (y == 0 || matrix[x].as_bytes()[y - 1] > val)
                && (y == matrix[x].len() - 1 || matrix[x].as_bytes()[y + 1] > val)
            {
                Some((x, y))
            } else {
                None
            }
        })
    });

    let risk_sums = low_points
        .clone()
        .map(|(x, y)| (matrix[x].as_bytes()[y] - b'0') as u64 + 1)
        .sum::<u64>();

    println!("part1: {}", risk_sums);

    let mut visit_count = vec![vec![0; matrix[0].len()]; matrix.len()];
    let top_3_mult = low_points
        .map(|(x, y)| {
            let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];
            let mut basin_size = 0;

            let mut queue = VecDeque::new();
            queue.push_back((x, y));

            while let Some((x, y)) = queue.pop_front() {
                if visited[x][y] {
                    continue;
                }

                let value = matrix[x].as_bytes()[y];

                if value == b'9' {
                    continue;
                }

                visited[x][y] = true;
                visit_count[x][y] += 1;
                basin_size += 1;
                assert!(visit_count[x][y] == 1);

                if x > 0 && !visited[x - 1][y] && matrix[x - 1].as_bytes()[y] > value {
                    queue.push_back((x - 1, y));
                }

                if x + 1 < visited.len()
                    && !visited[x + 1][y]
                    && matrix[x + 1].as_bytes()[y] > value
                {
                    queue.push_back((x + 1, y));
                }

                if y > 0 && !visited[x][y - 1] && matrix[x].as_bytes()[y - 1] > value {
                    queue.push_back((x, y - 1));
                }

                if y + 1 < visited[x].len()
                    && !visited[x][y + 1]
                    && matrix[x].as_bytes()[y + 1] > value
                {
                    queue.push_back((x, y + 1));
                }
            }
            println!("{:?} {}", (x, y), basin_size);

            basin_size
        })
        .sorted_by(|a, b| a.cmp(b).reverse())
        .take(3)
        .product::<usize>();

    println!("part2: {}", top_3_mult);
}
