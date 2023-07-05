fn bit_at_location(input: &usize, index: usize) -> usize {
    input >> index & 1
}

fn update_counts(bit_count: Vec<i32>, bits: usize) -> Vec<i32> {
    bit_count
        .into_iter()
        .enumerate()
        .map(|(idx, n)| {
            if bit_at_location(&bits, idx) == 1 { n + 1 } else { n - 1 }
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let width = input.lines().next().unwrap().chars().count();
    let mask = (1 << width) - 1;
    let gamma = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .fold(vec![0; width], update_counts)
        .into_iter()
        .enumerate()
        .map(|(idx, count)| ((count > 0) as u32) << idx)
        .sum::<u32>();
    let epsilon = !gamma & mask;
    (gamma * epsilon) as usize
}

fn find_most_common(idx: usize, set: &Vec<usize>) -> usize {
    assert!(!set.is_empty());
    let (ones, zeroes): (Vec<usize>, Vec<usize>) =
        set.iter().partition(|n| bit_at_location(n, idx) == 1);
    let largest = if ones.len() >= zeroes.len() { ones } else { zeroes };
    if largest.len() == 1 {
        return largest[0];
    }
    find_most_common(idx - 1, &largest)
}

fn find_least_common(idx: usize, set: &Vec<usize>) -> usize {
    assert!(!set.is_empty());
    let (ones, zeroes): (Vec<usize>, Vec<usize>) =
        set.iter().partition(|n| bit_at_location(n, idx) == 1);
    let smallest = if ones.len() >= zeroes.len() { zeroes } else { ones };
    if smallest.len() == 1 {
        return smallest[0];
    }
    find_least_common(idx - 1, &smallest)
}

pub fn part2(input: &str) -> usize {
    let width = input.lines().next().unwrap().chars().count();
    // let mask = (1 << width) - 1;
    let mapped = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();
    let oxygen = find_most_common(width - 1, &mapped);
    let co2 = find_least_common(width - 1, &mapped);
    oxygen * co2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1(include_str!("../resources/example_input.txt")), 198);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(include_str!("../resources/example_input.txt")), 230);
    }
}
