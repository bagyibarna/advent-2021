#[derive(Debug, Clone)]
enum Number {
    Val(u64),
    List(Box<[Number; 2]>),
}

struct Explosion {
    correct_left: Option<u64>,
    correct_right: Option<u64>,
}

impl Number {
    fn correct_index(&mut self, i: usize, m: u64) -> bool {
        match self {
            Number::Val(n) => {
                *n += m;
                true
            }
            Number::List(arr) => arr[i].correct_index(i, m),
        }
    }

    fn explosion_from(&mut self) -> Explosion {
        match self {
            Number::List(array) => match array.as_mut() {
                [Number::Val(left), Number::Val(right)] => {
                    let explosion = Explosion {
                        correct_left: Some(*left),
                        correct_right: Some(*right),
                    };

                    *self = Number::Val(0);
                    explosion
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    fn explode(&mut self, depth: usize) -> Option<Explosion> {
        match self {
            Number::Val(_) => None,
            Number::List(numbers) => {
                if depth == 4 {
                    Some(self.explosion_from())
                } else {
                    let mut explosion = numbers[0].explode(depth + 1);
                    if let Some(explosion) = &mut explosion {
                        if let Some(correction) = explosion.correct_right {
                            if numbers[1].correct_index(0, correction) {
                                explosion.correct_right = None
                            }
                        }
                    } else {
                        explosion = numbers[1].explode(depth + 1);
                        if let Some(explosion) = &mut explosion {
                            if let Some(correction) = explosion.correct_left {
                                if numbers[0].correct_index(1, correction) {
                                    explosion.correct_left = None;
                                }
                            }
                        }
                    }

                    explosion
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            &mut Number::Val(n) => {
                if n >= 10 {
                    let half = n / 2;
                    *self = Number::List(Box::new([Number::Val(half), Number::Val(half + n % 2)]));
                    true
                } else {
                    false
                }
            }
            Number::List(arr) => arr[0].split() || arr[1].split(),
        }
    }

    fn reduce(&mut self) {
        while self.explode(0).is_some() || self.split() {}
    }

    fn magnitude(&self) -> u64 {
        match self {
            &Number::Val(n) => n,
            Number::List(numbers) => numbers[0].magnitude() * 3 + numbers[1].magnitude() * 2,
        }
    }
}

impl std::str::FromStr for Number {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<u64>() {
            Ok(Number::Val(n))
        } else {
            let mut position = 0;
            let mut depth = 0;
            for (i, ch) in s.char_indices() {
                match ch {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' if depth == 1 => {
                        position = i;
                        break;
                    }
                    _ => {}
                }
            }
            Ok(Number::List(Box::new([
                s[1..position].parse()?,
                s[position + 1..s.len() - 1].parse()?,
            ])))
        }
    }
}

impl std::ops::Add<Number> for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Self::Output {
        let mut result = Number::List(Box::new([self, rhs]));
        result.reduce();
        result
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Number::Val(n) => n.fmt(f),
            Number::List(arr) => {
                f.write_str("[")?;
                arr[0].fmt(f)?;
                f.write_str(",")?;
                arr[1].fmt(f)?;
                f.write_str("]")
            }
        }
    }
}

fn found_max_sum(numbers: &[Number]) -> u64 {
    let mut max = u64::MIN;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                max = max.max((numbers[i].clone() + numbers[j].clone()).magnitude())
            }
        }
    }

    max
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let numbers = content
        .lines()
        .flat_map(|n| n.parse::<Number>())
        .collect::<Vec<_>>();

    let part1 = numbers[1..]
        .iter()
        .fold(numbers[0].clone(), |acc, n| acc + n.clone())
        .magnitude();

    println!("{}", part1);

    println!("{}", found_max_sum(&numbers));
}
