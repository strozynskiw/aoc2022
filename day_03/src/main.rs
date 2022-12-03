use std::fs;
use std::time::Instant;
use itertools::Itertools;

type Generated = Vec<String>;

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect()
}

fn part_1(input: &Generated) -> i32 {
input.iter()
.map(|l| (String::from_iter(l.chars().take(l.len()/2)), String::from_iter(l.chars().skip(l.len()/2))))
.map(|(s1, s2)| {
    for s1_char in s1.chars() {
        for s2_char in s2.chars() {
            if s1_char == s2_char {
                return s1_char
            }
        }
    }
    panic!("No match!");
} )
.map(|char| {
    match char {
        ('a'..='z') => 1 + char as i32 - 'a' as i32,
        ('A'..='Z') => 27 + char as i32 - 'A' as i32,
        _ => panic!("Incorrect char"),
    }
})
.sum()

}


fn part_2(input: &Generated) -> i32 {
input.iter().chunks(3).into_iter().map(|mut chunk| {
    let s1 = chunk.next().unwrap();
    let s2 = chunk.next().unwrap();
    let s3 = chunk.next().unwrap();
    for c in s1.chars() {
        if s2.contains(c) && s3.contains(c) {
            return c;
        }
    }
    panic!("No match!");
})
.map(|char| {
    match char {
        ('a'..='z') => 1 + char as i32 - 'a' as i32,
        ('A'..='Z') => 27 + char as i32 - 'A' as i32,
        _ => panic!("Incorrect char"),
    }
})
.sum()
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
        assert_eq!(157, part_1(&generate("vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw")));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(70, part_2(&generate("vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw")));
    }
}
