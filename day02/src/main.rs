use day02::{part1, part2};

fn main() {
    let input = include_str!("../resources/input.txt");
    let result1 = part1(input);
    println!("The answer to part 1 is: {result1}");
    let result2 = part2(input);
    println!("The answer to part 2 is: {result2}");
}
