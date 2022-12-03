use std::fs;
use std::time::Instant;

type Generated = Vec<(char, char)>;

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| (l.chars().nth(0).unwrap(), l.chars().nth(2).unwrap()))
        .collect()
}

fn part_1(input: &Generated) -> i32 {
input.iter().map(|(other, mine)| {
    let outcome_points = match (other, mine) {
        ('A', 'X') => 3,
        ('A', 'Y') => 6,
        ('A', 'Z') => 0,

        ('B', 'X') => 0,
        ('B', 'Y') => 3,
        ('B', 'Z') => 6,

        ('C', 'X') => 6,
        ('C', 'Y') => 0,
        ('C', 'Z') => 3,
        _ => panic!("Incorrect combination"),
    };

    let shape_points = match mine {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Incorrect shape"),
    };
    shape_points + outcome_points
}).sum()
}


fn part_2(input: &Generated) -> i32 {
    input.iter().map(|(other, mine)| {
        match (other, mine) {
            ('A', 'X') => 0+3,
            ('A', 'Y') => 3+1,
            ('A', 'Z') => 6+2,

            ('B', 'X') => 0+1,
            ('B', 'Y') => 3+2,
            ('B', 'Z') => 6+3,

            ('C', 'X') => 0+2,
            ('C', 'Y') => 3+3,
            ('C', 'Z') => 6+1,
            _ => panic!("Incorrect combination"),
        }
    }).sum()
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");

    let data = generate(&content);

    let res1_start = Instant::now();
    let res1 = part_1(&data);
    let res1_stop = Instant::now();

    let res2_start = Instant::now();
    let res2 = part_2(&data);
    let res2_stop = Instant::now();

    print!("Result1: {}\nResolved in: {:?}\n", res1, res1_stop.duration_since(res1_start));
    print!("Result2: {}\nResolved in: {:?}\n", res2, res2_stop.duration_since(res2_start));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(15, part_1(&generate(
"A Y
B X
C Z
")));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(12, part_2(&generate(
"A Y
B X
C Z")));
    }
}
