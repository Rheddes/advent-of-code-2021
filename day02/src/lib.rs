
pub fn part1(input: &str) -> isize {
    let coords = input.lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold((0, 0), |(h, d), (command, n)| {
            match (command, n.parse::<isize>().unwrap()) {
                ("forward", v) => (h + v, d),
                ("down", v) => (h, d + v),
                ("up", v) => (h, d - v),
                _ => unreachable!(),
            }
        });
    coords.0 * coords.1
}

pub fn part2(input: &str) -> isize {
    let coords = input.lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold((0, 0, 0), |(horizontal, depth, aim), (command, n)| {
            match (command, n.parse::<isize>().unwrap()) {
                ("forward", v) => (horizontal + v, depth + (v * aim), aim),
                ("down", v) => (horizontal, depth, aim + v),
                ("up", v) => (horizontal, depth, aim - v),
                _ => unreachable!(),
            }
        });
    coords.0 * coords.1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1(include_str!("../resources/example_input.txt")), 150);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(include_str!("../resources/example_input.txt")), 900);
    }
}