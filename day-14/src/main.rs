use std::collections::HashMap;

const fn repr(u: u8) -> u8 {
    u - b'A'
}

const MAX_VALUE: u8 = repr(b'Z');
const VALUE_COUNT: usize = (MAX_VALUE + 1) as usize;

fn part2(start: &str, rules: &HashMap<(u8, u8), u8>, iterations: usize) {
    let mut table = vec![[[0; VALUE_COUNT]; VALUE_COUNT]; iterations + 1];

    for window in start.as_bytes().windows(2) {
        table[0][repr(window[0]) as usize][repr(window[1]) as usize] += 1;
    }

    for i in 0..iterations {
        for left in 0..VALUE_COUNT {
            for right in 0..VALUE_COUNT {
                if let Some(&insert) = rules.get(&(left as u8, right as u8)) {
                    let count = table[i][left][right];
                    table[i + 1][left][insert as usize] += count;
                    table[i + 1][insert as usize][right] += count;
                }
            }
        }
    }

    let mut min = usize::MAX;
    let mut max = usize::MIN;
    let mut sum = 0;
    for (i, mut count) in table[iterations]
        .iter()
        .map(|arr| arr.iter().sum::<usize>())
        .enumerate()
    {
        if count == 0 {
            continue;
        }

        if i == repr(start.bytes().rev().next().unwrap()) as usize {
            count += 1;
        }

        min = min.min(count);
        max = max.max(count);
        sum += count;
    }
    println!("part2: {} ({})", max - min, sum);
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = content.lines();

    let start = lines.next().unwrap();
    let rules = lines
        .skip(1)
        .flat_map(|line| {
            let mut words = line.split_ascii_whitespace();
            let left = words.next()?;
            let right = words.nth(1)?;

            unsafe {
                Some((
                    left.as_bytes().get_unchecked(0) - b'A',
                    left.as_bytes().get_unchecked(1) - b'A',
                    right.as_bytes().get_unchecked(0) - b'A',
                ))
            }
        })
        .fold(HashMap::new(), |mut map, (a, b, res)| {
            map.insert((a, b), res);
            map
        });

    part2(start, &rules, 40);
    //part2(start, rules);
}
