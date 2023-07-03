pub fn part1(input: &str) -> usize {
    input.lines()
        .map(|line| { line.parse().unwrap() })
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

pub fn part2(input: &str) -> usize {
    input.lines()
        .map(|line| { line.parse().unwrap() })
        .collect::<Vec<usize>>()
        .windows(4)
        .filter(|window| window[0] < window[3])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1(include_str!("../resources/example_input.txt")), 7);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(include_str!("../resources/example_input.txt")), 5);
    }
}
