const BIT_COUNT: usize = 12;

fn update_common_counter(mut counter: [i16; BIT_COUNT], number: &u32) -> [i16; BIT_COUNT] {
    for i in 0..BIT_COUNT {
        let bit = (number & (1 << i)) >> i;
        counter[BIT_COUNT - 1 - i] += bit as i16 * 2 - 1;
    }

    counter
}

fn gamma_epsilon(counts: [i16; BIT_COUNT]) -> (u32, u32) {
    let mut decoded: u32 = 0;

    for (i, count) in counts.iter().enumerate() {
        decoded |= if *count > 0 { 1 } else { 0 } << (BIT_COUNT - 1 - i);
    }
    (decoded, !decoded & ((1 << BIT_COUNT) - 1))
}

fn split_most_least_common(numbers: &mut [u32], bit: usize) -> (&mut [u32], &mut [u32]) {
    let mut start = 0;
    let mut end = numbers.len() - 1;

    let mask = 1 << bit;

    'outer: while start < end {
        while numbers[start] & mask == 0 {
            start += 1;
            if start >= end {
                break 'outer;
            }
        }

        while numbers[end] & mask != 0 {
            end -= 1;
            if start >= end {
                break 'outer;
            }
        }

        numbers.swap(start, end);

        start += 1;
        end -= 1;
    }

    let mid = if start == end && numbers[start] & mask == 0 {
        start + 1
    } else {
        start
    };

    let (zeros, ones) = numbers.split_at_mut(mid);

    // ties are intentional
    if zeros.len() > ones.len() {
        (zeros, ones)
    } else {
        (ones, zeros)
    }
}

fn most_least_commons_bitwise(mut numbers: Vec<u32>) -> (u32, u32) {
    let (mut most_common, mut least_common) = split_most_least_common(&mut numbers, BIT_COUNT - 1);

    for bit in (0..BIT_COUNT - 1).rev() {
        most_common = split_most_least_common(most_common, bit).0;
        if most_common.len() == 1 {
            break;
        }
    }

    for bit in (0..BIT_COUNT - 1).rev() {
        least_common = split_most_least_common(least_common, bit).1;
        if least_common.len() == 1 {
            break;
        }
    }

    (most_common[0], least_common[0])
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let numbers = content
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    let counts = numbers.iter().fold([0; BIT_COUNT], update_common_counter);
    let (gamma, epsilon) = gamma_epsilon(counts);
    println!("part1: {}", gamma * epsilon);

    let (oxygen, co2) = most_least_commons_bitwise(numbers);
    println!("part2: {}", oxygen as u32 * co2 as u32);
}
