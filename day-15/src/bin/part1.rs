fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn hash(to_hash: &str) -> usize {
    to_hash.chars().fold(0, |current_value, c| {
        if c == '\n' {
            return current_value;
        }
        let mut next_value = current_value + c as usize;
        next_value *= 17;
        next_value %= 256;
        next_value
    })
}

fn part1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn part1_test() {
        let result = part1("\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
        );
        assert_eq!(result, 1320);
    }
}
