fn main() {
    let mut content = std::fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<i64>>();

    content.sort_unstable();
    let middle = content[content.len() / 2];
    let part1: i64 = content.iter().map(|&n| (n - middle).abs()).sum();

    println!("part1: {}", part1);

    // no idiomatic way to use binary search
    let part2: i64 = (0..content[content.len() - 1])
        .map(|n| {
            content
                .iter()
                .map(|&m| {
                    let diff = (m - n).abs();
                    (diff * (diff + 1)) / 2
                })
                .sum()
        })
        .min()
        .unwrap();

    println!("part2: {}", part2);
}
