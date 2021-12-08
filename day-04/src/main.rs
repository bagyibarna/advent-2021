use std::{
    collections::{HashMap, HashSet},
    error::Error,
    iter::Iterator,
};

use itertools::Itertools;

#[derive(Default, Clone, Copy)]
struct Line {
    nums: [u32; 5],
    count: u32,
    found: [bool; 5],
}

#[derive(Default, Clone, Copy)]
struct Board {
    rows: [Line; 5],
    cols: [Line; 5],
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Finder {
    board: usize,
    row: usize,
    col: usize,
}

impl Board {
    fn new<RowIt: Iterator<Item = u32>, It: Iterator<Item = RowIt>>(
        mapper: &mut HashMap<u32, Vec<Finder>>,
        board: usize,
        it: It,
    ) -> Self {
        let mut res = Board::default();
        for (row, row_content) in it.enumerate() {
            for (col, num) in row_content.enumerate() {
                res.rows[row].nums[col] = num;
                res.cols[col].nums[row] = num;
                mapper
                    .entry(num)
                    .or_default()
                    .push(Finder { board, row, col })
            }
        }

        res
    }
}

fn calculate_score(board: &Board) -> u32 {
    board.rows.iter().fold(0, |n, row| {
        n + row
            .nums
            .iter()
            .zip(row.found.iter())
            .filter(|(_, &f)| !f)
            .map(|(n, _)| n)
            .sum::<u32>()
    })
}

fn play(
    mapper: &mut HashMap<u32, Vec<Finder>>,
    boards: &mut Vec<Board>,
    numbers: &[u32],
) -> (u32, u32) {
    let mut first_win = None;
    let mut last_win = None;
    let mut won = HashSet::with_capacity(boards.len());
    for num in numbers {
        if let Some(places) = mapper.get(num) {
            for place in places {
                let board = &mut boards[place.board];
                if won.contains(&place.board) {
                    continue;
                }

                let row = &mut board.rows[place.row];
                let col = &mut board.cols[place.col];

                if row.found[place.col] {
                    continue;
                }

                row.found[place.col] = true;
                col.found[place.row] = true;

                row.count += 1;
                col.count += 1;
                if row.count == 5 || col.count == 5 {
                    won.insert(place.board);
                    let score = calculate_score(board) * num;
                    if first_win == None {
                        first_win = Some(score);
                    }
                    last_win = Some(score);
                }
            }
        }
    }

    (first_win.unwrap(), last_win.unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("input.txt")?;
    let mut lines = content.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split_terminator(',')
        .map(|num_str| num_str.parse().unwrap())
        .collect::<Vec<_>>();

    let mut mapper = HashMap::new();

    let mut boards = lines
        .chunks(6)
        .into_iter()
        .enumerate()
        .map(|(board, board_lines_it)| {
            let content = board_lines_it.skip(1).map(|board_line| {
                board_line
                    .split_ascii_whitespace()
                    .map(|board_num| board_num.parse().unwrap())
            });

            Board::new(&mut mapper, board, content)
        })
        .collect::<Vec<_>>();

    println!("winner:{:?}", play(&mut mapper, &mut boards, &numbers));

    Ok(())
}
