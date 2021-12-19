fn simulate(mut v_x: i64, mut v_y: i64, x1: i64, x2: i64, y1: i64, y2: i64) -> bool {
    let mut curr_x = 0;
    let mut curr_y = 0;

    loop {
        if curr_x > x2 || curr_y < y1 {
            return false;
        }

        if curr_x >= x1 && curr_y <= y2 {
            return true;
        }

        curr_x += v_x;
        curr_y += v_y;
        v_x -= v_x.signum();
        v_y -= 1;
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let (x1, x2, y1, y2) = scan_fmt::scan_fmt!(
        &content,
        "target area: x={d}..{d}, y={d}..{d}",
        i64,
        i64,
        i64,
        i64
    )
    .unwrap();

    let max_y = (-y1 * (-y1 - 1)) / 2;

    println!("part1: {}", max_y);

    let mut count = 0;

    for v_x in 0..=x2 {
        for v_y in y1..=max_y {
            count += simulate(v_x, v_y, x1, x2, y1, y2) as i64;
        }
    }

    println!("part2: {}", count);
}
