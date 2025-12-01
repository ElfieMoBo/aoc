use std::fs;

mod day01;

fn main() {
    let input_01 = fs::read_to_string(&"inputs/day01.txt").expect("Failed to read input file");
    let res011 = day01::part_1(&input_01);
    println!("----- DAY 01 -----");
    println!("day 01.1: {}", res011);
    let res012 = day01::part_2(&input_01);
    println!("day 01.2: {}", res012);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_1() {
        let test_file = fs::read_to_string("inputs/day01_test.txt").unwrap();
        assert_eq!(day01::part_1(&test_file), 3);
        assert_eq!(day01::part_2(&test_file), 6);
    }
}
