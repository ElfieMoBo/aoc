use std::fs;
macro_rules! set_day {
    ( $module:ident ) => {
        // Import mod of the day
        mod $module;
        #[cfg(test)]
        use $module::get_part_1;
        #[cfg(test)]
        use $module::get_part_2;
        use $module::part_1;
        use $module::part_2;
        static DAY: &str = stringify!($module);
    };
}

set_day!(day05);

fn main() {
    let input =
        fs::read_to_string(format!("inputs/{}.txt", DAY)).expect("Failed to read input file");
    let part1 = part_1(&input);
    println!("----- {} -----", DAY);
    println!("part 1: {}", part1);
    let part2 = part_2(&input);
    println!("part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day() {
        let test_file = fs::read_to_string(format!("inputs/{}_test.txt", DAY)).unwrap();
        assert_eq!(part_1(&test_file), get_part_1());
        assert_eq!(part_2(&test_file), get_part_2());
    }
}
