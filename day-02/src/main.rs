fn parse_line(line: &str) -> (i64, i64) {
    let mut words = line.split_ascii_whitespace();
    let dir = words.next().unwrap();
    let val = words.next().unwrap().parse().unwrap();
    match dir {
        "forward" => (val, 0),
        "up" => (0, -val),
        "down" => (0, val),
        _ => panic!(),
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let parsed = content.lines().map(parse_line).collect::<Vec<_>>();

    let end1 = parsed
        .iter()
        .fold((0, 0), |(x, y), &(dx, dy)| (x + dx, y + dy));
    println!("part1: {}", end1.0 * end1.1);

    let end2 = parsed.iter().fold((0, 0, 0), |(aim, x, y), &(dx, daim)| {
        let new_aim = aim + daim;
        (new_aim, x + dx, y + dx * new_aim)
    });
    println!("part2: {}", end2.1 * end2.2);
}
