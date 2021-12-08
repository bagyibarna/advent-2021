fn simulate(mut fishtank: [usize; 9], rounds: usize) -> usize {
    for _ in 0..rounds {
        fishtank[7] += fishtank[0];
        fishtank.rotate_left(1);
    }

    fishtank.iter().sum()
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let fishtank = content
        .split_terminator(',')
        .map(|n| n.parse().unwrap())
        .fold([0; 9], |mut fish_count, num: usize| {
            fish_count[num] += 1;
            fish_count
        });

    let result = simulate(fishtank, 256);
    println!("fish count: {}", result);
}
