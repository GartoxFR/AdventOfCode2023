fn main() {
    let input = include_str!("../../input1.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find(|char| char.is_ascii_digit()).unwrap();
            let last = line
                .chars()
                .rev()
                .find(|char| char.is_ascii_digit())
                .unwrap();

            first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(solve(input), "142");
    }
}
