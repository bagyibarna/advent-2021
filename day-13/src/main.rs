use std::collections::HashSet;

fn parse_point(s: &str) -> Option<(usize, usize)> {
    if let Some((a, b)) = s.split_once(',') {
        if let (Ok(a), Ok(b)) = (a.parse(), b.parse()) {
            return Some((a, b));
        }
    }

    None
}

fn parse_fold(s: &str) -> (bool, usize) {
    let mut words = s.split_ascii_whitespace().skip(2);
    let (axis, n) = words.next().unwrap().split_once('=').unwrap();
    (axis == "y", n.parse().unwrap())
}

fn apply_folds(folds: &[(bool, usize)], (mut x, mut y): (usize, usize)) -> (usize, usize) {
    for &(y_fold, n) in folds {
        let modified = if y_fold { &mut y } else { &mut x };
        if *modified > n {
            *modified = 2 * n - *modified;
        }
    }

    (x, y)
}

fn point_count(folds: &[(bool, usize)], coords: &[(usize, usize)]) -> usize {
    coords
        .iter()
        .fold(HashSet::new(), |mut set, &coord| {
            set.insert(apply_folds(folds, coord));
            set
        })
        .len()
}

fn print_fold_result(extents: (usize, usize), folds: &[(bool, usize)], coords: &[(usize, usize)]) {
    let mut new_max_y = 0;
    let mut new_max_x = 0;

    let table = coords.iter().fold(
        vec![vec![false; extents.1]; extents.0],
        |mut table, &coord| {
            let (x, y) = apply_folds(folds, coord);
            new_max_x = new_max_x.max(x);
            new_max_y = new_max_y.max(y);

            table[x][y] = true;
            table
        },
    );

    for row in &table[..=new_max_x] {
        for &a in &row[..=new_max_y] {
            print!("{}", if a { "#" } else { "." });
        }
        println!();
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut lines = content.lines();

    let mut coords = Vec::new();
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    while let Some(coord) = parse_point(lines.next().unwrap()) {
        coords.push(coord);
        max_x = max_x.max(coord.0);
        max_y = max_y.max(coord.1);
    }

    let folds = lines.map(parse_fold).collect::<Vec<_>>();

    let part1 = point_count(&folds[0..1], &coords);
    println!("part1: {}", part1);
    print_fold_result((max_x, max_y), &folds, &coords);
}
