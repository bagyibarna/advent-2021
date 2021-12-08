use itertools::Itertools;

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let values = content.lines().map(|line| line.parse().unwrap()).collect::<Vec<i64>>();

    let part1 = values.iter().tuple_windows().filter(|(a, b)| a < b).count();
    let part2 = values.iter().tuple_windows().filter(|(a, _, _, b)| a < b ).count();

    println!("part1 {}", part1);
    println!("part2 {}", part2);
}
