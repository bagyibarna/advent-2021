const ERROR_SCORE_TABLE: [usize; 256] = {
    let mut table = [0; 256];
    table[b')' as usize] = 3;
    table[b']' as usize] = 57;
    table[b'}' as usize] = 1197;
    table[b'>' as usize] = 25137;

    table
};

const COMPLETE_SCORE_TABLE: [usize; 256] = {
    let mut table = [0; 256];
    table[b'(' as usize] = 1;
    table[b'[' as usize] = 2;
    table[b'{' as usize] = 3;
    table[b'<' as usize] = 4;

    table
};

const OPEN_TABLE: [u8; 256] = {
    let mut table = [0; 256];
    table[b')' as usize] = b'(';
    table[b']' as usize] = b'[';
    table[b'}' as usize] = b'{';
    table[b'>' as usize] = b'<';

    table
};

enum Score {
    Complete(usize),
    Error(usize),
}

fn score(line: &str) -> Score {
    let mut stack = Vec::new();
    for &c in line.as_bytes() {
        match c {
            b'(' | b'[' | b'{' | b'<' => {
                stack.push(c);
            }
            b')' | b']' | b'}' | b'>' => {
                if stack.last() == Some(&OPEN_TABLE[c as usize]) {
                    stack.pop();
                } else {
                    return Score::Error(ERROR_SCORE_TABLE[c as usize]);
                }
            }
            _ => unreachable!(),
        }
    }

    Score::Complete(
        stack
            .iter()
            .rev()
            .fold(0, |acc, &c| acc * 5 + COMPLETE_SCORE_TABLE[c as usize]),
    )
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let error_scores = content
        .lines()
        .map(|line| (score(line), line))
        .collect::<Vec<_>>();

    let error_score_sum = error_scores
        .iter()
        .filter_map(|(score, _)| match score {
            Score::Error(n) => Some(n),
            _ => None,
        })
        .sum::<usize>();
    println!("part1: {}", error_score_sum);

    let mut complete_scores = error_scores
        .iter()
        .filter_map(|(score, _)| match score {
            Score::Complete(n) => Some(n),
            _ => None,
        })
        .collect::<Vec<_>>();

    let complete_count = complete_scores.len();

    let complete_median = *complete_scores.select_nth_unstable(complete_count / 2).1;
    println!("part2: {}", complete_median);
}
