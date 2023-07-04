use std::str::FromStr;

enum Command {
    Forward { n: isize },
    Up { n: isize },
    Down { n: isize },
}

#[derive(Debug)]
struct ParseCommandError;

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            Some(("forward", x)) => Ok(Self::Forward { n: x.parse().unwrap() }),
            Some(("up", x)) => Ok(Self::Up { n: x.parse().unwrap() }),
            Some(("down", x)) => Ok(Self::Down { n: x.parse().unwrap() }),
            Some(_) => Err(ParseCommandError),
            None => Err(ParseCommandError),
        }
    }
}

fn handle_part_1(command: Command, (horizontal, depth): (isize, isize)) -> (isize, isize) {
    match command {
        Command::Forward { n } => (horizontal + n, depth),
        Command::Down { n } => (horizontal, depth + n),
        Command::Up { n } => (horizontal, depth - n),
    }
}

fn handle_part_2(command: Command, (horizontal, depth, aim): (isize, isize, isize)) -> (isize, isize, isize) {
    match command {
        Command::Forward { n } => (horizontal + n, depth + n * aim, aim),
        Command::Down { n } => (horizontal, depth, aim + n),
        Command::Up { n } => (horizontal, depth, aim - n),
    }
}

pub fn part1(input: &str) -> isize {
    let coords = input.lines()
        .map(|line| Command::from_str(line))
        .fold((0, 0), |coords, command| {
            handle_part_1(command.unwrap(), coords)
        });
    coords.0 * coords.1
}

pub fn part2(input: &str) -> isize {
    let coords = input.lines()
        .map(|line| Command::from_str(line))
        .fold((0, 0, 0), |coords, command| {
            handle_part_2(command.unwrap(), coords)
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