use std::fs;
use std::time::Instant;

type Generated = Vec<(Section, Section)>;

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut iter = l.split(',');
            (iter.next().unwrap().into(), iter.next().unwrap().into())
        })
        .collect()
}

struct Section {
    start: u32,
    end: u32,
}

impl From<&str> for Section {
    fn from(input: &str) -> Self {
        // "a-b"
        let mut iter = input.split('-');
        Section {
            start: iter.next().unwrap().parse().unwrap(),
            end: iter.next().unwrap().parse().unwrap(),
        }
    }
}

fn part_1(input: &Generated) -> usize {
    input
        .iter()
        .filter(|(s1, s2)| {
            (s1.start <= s2.start && s1.end >= s2.end) || (s2.start <= s1.start && s2.end >= s1.end)
        })
        .count()
}

fn part_2(input: &Generated) -> usize {
    input
        .iter()
        .filter(|(s1, s2)| {
            (s2.start >= s1.start && s2.start <= s1.end)
                || (s2.end >= s1.start && s2.end <= s1.end)
                || (s1.start >= s2.start && s1.start <= s2.end)
                || (s1.end >= s2.start && s1.end <= s2.end)
        })
        .count()
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

    print!(
        "Result1: {}\nResolved in: {:?}\n",
        res1,
        res1_stop.duration_since(res1_start)
    );
    print!(
        "Result2: {}\nResolved in: {:?}\n",
        res2,
        res2_stop.duration_since(res2_start)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(
            2,
            part_1(&generate(
        "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"
            ))
        );
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            5,
            part_2(&generate(
        "2-4,6-8
        6-8,2-4
        2-3,4-5
        5-7,7-9
        2-8,3-7
        3-7,2-8
        6-6,4-6
        2-6,4-8"
            ))
        );
    }
}
