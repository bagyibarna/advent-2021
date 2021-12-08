/*
const PATTERNS: [(u8, &str); 10] = [
    (0, "abcefg"),  // iii iii  6 [6..8]
    (1, "cf"),      //   i  i   2 [0]
    (2, "acdeg"),   // i iii i  5 [3..5]
    (3, "acdfg"),   // i ii ii  5 [3..5]
    (4, "bcdf"),    //  iii i   4 [2]
    (5, "abdfg"),   // ii i ii  5 [3..5]
    (6, "abdefg"),  // ii iiii  6 [6..8]
    (7, "acf"),     // i i  i   3 [1]
    (8, "abcdefg"), // iiiiiii  7 [9]
    (9, "abcdfg"),  // iiii ii  6 [6..8]
]; */

// i iii i  2
// i ii ii  3
// ii i ii  5

//unique: cf, acf, bcdf,
//5s common: adg
//5s one missing: cf
//5s two missing: be

#[allow(clippy::many_single_char_names)]
fn solve_input<'a, T: Iterator<Item = &'a str>>(it: T) -> [u8; 7] {
    let mut processed = [(0, 0); 10];

    for (i, n_str) in it.enumerate() {
        let mut res: u8 = 0;
        for n in n_str.bytes() {
            res |= 1 << (n - b'a');
        }
        processed[i] = (n_str.len(), res);
    }

    processed.sort_unstable_by_key(|a| a.0);

    let cf = processed[0].1;
    let acf = processed[1].1;
    let adg = processed[3].1 & processed[4].1 & processed[5].1;
    let be = !acf & !adg & 0b1111111;
    let cde = !(processed[6].1 & processed[7].1 & processed[8].1) & 0b1111111;

    let a = acf & !cf;
    let b = processed[2].1 & be;
    let e = be & !b;
    let g = adg & !(a | processed[2].1);
    let d = adg & !(a | g);
    let c = cde & !(d | e);
    let f = cf & !c;

    let mut decode_map = [0; 7];
    decode_map[a.trailing_zeros() as usize] = 0;
    decode_map[b.trailing_zeros() as usize] = 1;
    decode_map[c.trailing_zeros() as usize] = 2;
    decode_map[d.trailing_zeros() as usize] = 3;
    decode_map[e.trailing_zeros() as usize] = 4;
    decode_map[f.trailing_zeros() as usize] = 5;
    decode_map[g.trailing_zeros() as usize] = 6;

    decode_map
}

fn decode<'a, T: Iterator<Item = &'a str>>(
    decode_map: [u8; 7],
    display_map: [u8; 256],
    it: T,
) -> u64 {
    it.map(|n_str| {
        let num_display_bits = n_str.bytes().fold(0, |acc, n| {
            let decoded = decode_map[(n - b'a') as usize];
            acc | (1 << decoded)
        });
        display_map[num_display_bits as usize] as u64
    })
    .fold(0, |acc, n| acc * 10 + n)
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let processed = content.lines().map(|line| {
        let (input, output) = line.split_once('|').unwrap();
        (
            input.split_ascii_whitespace(),
            output.split_ascii_whitespace(),
        )
    });

    let part1 = processed
        .clone()
        .flat_map(|line| line.1)
        .filter(|word| [2, 3, 4, 7].iter().any(|&a| a == word.len()))
        .count();

    println!("part1: {}", part1);

    let mut display_map = [u8::MAX; 256];
    display_map[0b1110111] = 0;
    display_map[0b0100100] = 1;
    display_map[0b1011101] = 2;
    display_map[0b1101101] = 3;
    display_map[0b0101110] = 4;
    display_map[0b1101011] = 5;
    display_map[0b1111011] = 6;
    display_map[0b0100101] = 7;
    display_map[0b1111111] = 8;
    display_map[0b1101111] = 9;

    let part2: u64 = processed
        .map(|(input, output)| {
            let decode_map = solve_input(input);

            decode(decode_map, display_map, output)
        })
        .sum();

    println!("part2: {}", part2);
}
