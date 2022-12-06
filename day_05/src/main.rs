use std::fs;
use std::time::Instant;

struct Instruction {
    m: usize,
    f: usize,
    t: usize,
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        //move 1 from 2 to 1
        let mut iter = line.split(' ');
        iter.next();
        let m = iter.next().unwrap().parse().unwrap();
        iter.next();
        let f = iter.next().unwrap().parse().unwrap();
        iter.next();
        let t = iter.next().unwrap().parse().unwrap();
        Instruction { m , f , t }
    }
}

type Generated = (Vec<Vec<char>>, Vec<Instruction>);

fn generate(input: &str) -> Generated {

    let buffer = vec![
        vec!['W', 'D', 'G', 'B', 'H', 'R', 'V'],
        vec!['J', 'N', 'G', 'C', 'R', 'F'],
        vec!['L', 'S', 'F', 'H', 'D', 'N', 'J'],
        vec!['J', 'D', 'S', 'V'],
        vec!['S', 'H', 'D', 'R', 'Q', 'W', 'N', 'V'],
        vec!['P', 'G', 'H', 'C', 'M'],
        vec!['F', 'J', 'B', 'G', 'L', 'Z', 'H', 'C'],
        vec!['S', 'J', 'R'],
        vec!['L', 'G', 'S', 'R', 'B', 'N', 'V', 'M'],
    ];

    dbg!(&buffer);

    let instructions = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.into())
        .collect();

    (buffer, instructions)
}

fn part_1(input: &Generated) -> String {
    let mut buffer = input.0.clone();
    for Instruction { m, f, t } in &input.1 {
        for _ in 0..*m {
            let cr = buffer.get_mut(*f-1).unwrap().pop().unwrap();
            buffer.get_mut(*t-1).unwrap().push(cr);
        }
    }
    buffer.iter().map(|stack| stack.last().unwrap_or(&'-')).collect::<String>()
}


fn part_2(input: &Generated) -> String {
    let mut buffer = input.0.clone();
    for Instruction { m, f, t } in &input.1 {
        let from_stack = buffer.get_mut(*f-1).unwrap();
        let len = from_stack.len();
        let part: Vec<char> = from_stack.drain(len-m..).collect();
        let to_stack = buffer.get_mut(*t-1).unwrap();
        to_stack.extend(part.iter());

    }
    buffer.iter().map(|stack| stack.last().unwrap_or(&'-')).collect::<String>()
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
        assert_eq!(0, part_1(&generate("")));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(0, part_2(&generate("")));
    }
}
