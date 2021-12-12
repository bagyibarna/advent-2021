use std::collections::VecDeque;

fn simulate_step(matrix: &mut Vec<Vec<u32>>) -> usize {
    let mut queue = VecDeque::new();

    for (x, row) in matrix.iter_mut().enumerate() {
        for (y, cell) in row.iter_mut().enumerate() {
            *cell += 1;
            if *cell == 10 {
                queue.push_back((x, y));
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        if x > 0 {
            matrix[x - 1][y] += 1;
            if matrix[x - 1][y] == 10 {
                queue.push_back((x - 1, y));
            }
            if y > 0 {
                matrix[x - 1][y - 1] += 1;
                if matrix[x - 1][y - 1] == 10 {
                    queue.push_back((x - 1, y - 1));
                }
            }
            if y < matrix.len() - 1 {
                matrix[x - 1][y + 1] += 1;
                if matrix[x - 1][y + 1] == 10 {
                    queue.push_back((x - 1, y + 1))
                }
            }
        }

        if x < matrix.len() - 1 {
            matrix[x + 1][y] += 1;
            if matrix[x + 1][y] == 10 {
                queue.push_back((x + 1, y));
            }
            if y > 0 {
                matrix[x + 1][y - 1] += 1;
                if matrix[x + 1][y - 1] == 10 {
                    queue.push_back((x + 1, y - 1));
                }
            }
            if y < matrix.len() - 1 {
                matrix[x + 1][y + 1] += 1;
                if matrix[x + 1][y + 1] == 10 {
                    queue.push_back((x + 1, y + 1))
                }
            }
        }

        if y > 0 {
            matrix[x][y - 1] += 1;
            if matrix[x][y - 1] == 10 {
                queue.push_back((x, y - 1))
            }
        }

        if y < matrix[0].len() - 1 {
            matrix[x][y + 1] += 1;
            if matrix[x][y + 1] == 10 {
                queue.push_back((x, y + 1))
            }
        }
    }

    let mut flashes = 0;
    for row in matrix {
        for cell in row {
            if *cell >= 10 {
                flashes += 1;
                *cell = 0;
            }
        }
    }

    flashes
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut matrix = content
        .lines()
        .map(|line| {
            line.bytes()
                .map(|ch| (ch - b'0') as u32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut flash_count = 0;
    let mut full_flash = None;
    for i in 0..100 {
        let count = simulate_step(&mut matrix);
        if full_flash.is_none() && count == matrix.len() * matrix[0].len() {
            full_flash = Some(i);
        }
        flash_count += count;
    }

    if full_flash.is_none() {
        for i in 100.. {
            let count = simulate_step(&mut matrix);
            if full_flash.is_none() && count == matrix.len() * matrix[0].len() {
                full_flash = Some(i);
                break;
            }
        }
    }

    println!("part1 {}", flash_count);
    println!("part2 {}", full_flash.unwrap() + 1);
}
