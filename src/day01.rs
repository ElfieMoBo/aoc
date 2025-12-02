fn parse_line(line: &str) -> (bool, i16) {
    let mut it = line.chars();
    let direction = it.next();
    let direction = match direction {
        Some('L') => true,
        Some('R') => false,
        _ => panic!("Invalid direction"),
    };
    let step: i16 = it.as_str().trim().parse().unwrap();
    (direction, step)
}

pub fn get_part_1() -> i16 {
    3
}

pub fn part_1(file: &String) -> i16 {
    // Count number of times the dial stop at 0
    let mut begin = 50;
    let mut counter = 0;
    for line in file.lines() {
        let (is_left, s) = parse_line(&line);
        begin = if is_left {
            (begin - s).rem_euclid(100)
        } else {
            (begin + s).rem_euclid(100)
        };
        if begin == 0 {
            counter += 1;
        }
    }
    counter
}

pub fn get_part_2() -> i16 {
    6
}

pub fn part_2(file: &String) -> i16 {
    // Count number of times the dial points once at 0 (during a rotation)
    let mut position = 50;
    let mut counter = 0;
    for line in file.lines() {
        let (is_left, s) = parse_line(&line);
        position = if is_left {
            if position == 0 {
                // In this case (beginning at 0 and going left/negative), it counts 0 twice
                counter -= 1;
            }
            position - s
        } else {
            position + s
        };

        let number = position.div_euclid(100);
        position = position.rem_euclid(100);

        if position == 0 && number <= 0 {
            // In this case, the euclidian division doesn't count the 0, otherwise yes
            counter += 1;
        }
        counter += number.abs();
    }
    counter
}
