use std::fs;
use std::time::Instant;

type Generated = Vec<i32>;

fn generate(input: &str) -> Generated {

    let split = input.split("\n\n").collect::<Vec<&str>>();

    dbg!(&split);

    split.iter().map(|calories| calories.lines().map(|line| line.parse::<i32>().unwrap()).sum()).collect()
}

fn part_1(input: &Generated) -> i32 {
    dbg!(input);
    *input.iter().max().unwrap()
}


fn part_2(input: &Generated) -> i32 {
    let mut input = input.clone();
    input.sort();
    dbg!(&input);
    input.iter().rev().take(3).sum()
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
        assert_eq!(24000, part_1(&generate(
&"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
        )))
    }
    #[test]
    fn test_part_2() {
        assert_eq!(45000, part_2(&generate(
&"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000")))
    }
}
