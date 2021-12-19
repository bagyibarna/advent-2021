#![allow(clippy::many_single_char_names)]

use nalgebra::*;
use std::collections::HashSet;

type Vector = Vector3<i32>;
type Matrix = Matrix3<i32>;

const ORIENTATION_INDICES: std::ops::Range<u32> = 0..((2 * 3) << 3) + 0b111 + 1;

fn permutation(ind: u32, x: i32, y: i32, z: i32) -> Matrix {
    let mut result = Matrix::from_diagonal(&Vector3::new(x, y, z));

    let mut n = ind >> 3;
    let first_index = n % 3;
    n /= 3;
    result.swap_rows(2, first_index as usize);
    let second_index = n & 1;
    result.swap_rows(1, second_index as usize);

    result
}

fn row_sign(ind: u32, i: usize) -> i32 {
    if (ind & (1 << i)) == 0 {
        1
    } else {
        -1
    }
}

fn transform(ind: u32) -> Matrix {
    permutation(ind, row_sign(ind, 0), row_sign(ind, 1), row_sign(ind, 2))
}

fn parse_scanners(s: &str) -> Vec<Vec<Vector>> {
    let mut result = Vec::new();
    let mut lines = s.lines();

    while let Some(line) = lines.next() {
        assert!(line.starts_with("---"));
        let mut new_scanner = Vec::new();
        for line in &mut lines {
            if let Ok((x, y, z)) = scan_fmt::scan_fmt!(line, "{d},{d},{d}", i32, i32, i32) {
                new_scanner.push(Vector::new(x, y, z));
            } else {
                break;
            }
        }
        result.push(new_scanner);
    }

    result
}

fn unification_method(origo: &HashSet<Vector>, scanner: &[Vector]) -> Option<(Matrix, Vector)> {
    for ind in ORIENTATION_INDICES {
        let transform = transform(ind);
        for ref_beacon in origo {
            for new_ind in 0..(scanner.len() - 11) {
                let dist = ref_beacon - transform * scanner[new_ind];
                let count = scanner[new_ind + 1..]
                    .iter()
                    .filter(|&&new_beacon| origo.contains(&(dist + transform * new_beacon)))
                    .take(11)
                    .count();

                if count == 11 {
                    return Some((transform, dist));
                }
            }
        }
    }

    None
}

fn unify_scanners(mut scanners: Vec<Vec<Vector>>) -> (HashSet<Vector>, i32) {
    let mut origo = HashSet::<Vector>::from_iter(scanners.pop().unwrap().into_iter());
    let mut scanner_pos = Vec::new();
    while !scanners.is_empty() {
        for scanner_ind in 0..scanners.len() {
            if let Some((mat, vec)) = unification_method(&origo, &scanners[scanner_ind]) {
                for &beacon in &scanners[scanner_ind] {
                    origo.insert(vec + mat * beacon);
                }
                scanner_pos.push(vec);
                scanners.swap_remove(scanner_ind);
                break;
            }
        }
    }

    let max_dist = scanner_pos
        .iter()
        .map(|x| {
            scanner_pos
                .iter()
                .map(|y| (x - y).abs().sum())
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    (origo, max_dist)
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let scanners = parse_scanners(&content);
    let (origo, max_dist) = unify_scanners(scanners);
    println!("part1: {}", origo.len());
    println!("part2: {}", max_dist);
}
