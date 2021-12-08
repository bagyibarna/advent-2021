#[macro_use]
extern crate scan_fmt;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let graph_it = input.lines().map(|line| {
        let (x1, y1, x2, y2) = scan_fmt!(line, "{},{} -> {},{}", i32, i32, i32, i32).unwrap();
        ((x1, y1), (x2, y2))
    });

    let matrix = graph_it.fold(
        vec![[(0, 0); 1000]; 1000],
        |mut vec, ((x1, y1), (x2, y2))| {
            let dir = ((x2 - x1), y2 - y1);
            let diag = dir.0 != dir.1;

            if dir.0.abs() != dir.1.abs() && dir.0 != 0 && dir.1 != 0 {
                return vec;
            }

            let walk = (dir.0.signum(), dir.1.signum());
            let mut curr = (x1, y1);

            loop {
                let cell = &mut vec[curr.0 as usize][curr.1 as usize];

                cell.1 += 1;
                if !diag {
                    vec[curr.0 as usize][curr.1 as usize].0 += 1;
                }

                if curr == (x2, y2) {
                    break;
                }

                curr.0 += walk.0;
                curr.1 += walk.1;
            }

            vec
        },
    );

    let overlapping =
        matrix
            .iter()
            .flatten()
            .fold((0, 0), |(nondiag_s, diag_s), (nondiag, diag)| {
                (
                    nondiag_s + if nondiag > &1 { 1 } else { 0 },
                    diag_s + if diag > &1 { 1 } else { 0 },
                )
            });

    println!("part1: {:?}", overlapping);
}
