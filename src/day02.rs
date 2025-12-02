fn parse_line(line: &str) -> Vec<(u64, u64)> {
    line
        .split(',')
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s| match s.split('-').collect::<Vec<_>>()[..] {
            [a, b] => (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()),
            _ => unreachable!(),
        })
        .collect()
}

pub fn get_part_1() -> u64 {
    1227775554
}

fn is_valid_1(n: u64) -> bool {
    if n < 10 {
        return false;
    }
    let n_str = n.to_string();
    let n_len = n_str.len();
    if n_len % 2 == 0 {
        return n_str[..(n_len/2)] == n_str[(n_len / 2)..];
    }
    false
}


pub fn part_1(file: &String) -> u64 {
    // Add numbers that are composed of some sequence of digits repeated twice
    parse_line(file)
        .into_iter()
        .map(|(a, b)| (a..=b).collect::<Vec<_>>().iter().filter(|n| is_valid_1(**n)).sum::<u64>())
        .collect::<Vec<_>>()
        .into_iter()
        .sum()
}

pub fn get_part_2() -> u64 {
    4174379265
}

fn is_valid_2(n: u64) -> bool {
    if n < 10 {
        return false;
    }
    let n_str = n.to_string();
    let n_len = n_str.len();
    for i in 1..n_len {
        if n_len % i == 0 {
            let slices = n_str.as_bytes()
                .chunks(i)
                .map(str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap();
            if slices.iter().max() == slices.iter().min() {
                return true;
            }
        }
    }
    false
}


pub fn part_2(file: &String) -> u64 {
    // Add numbers that are composed of some sequence of digits repeated at least twice
    parse_line(file)
        .into_iter()
        .map(|(a, b)| (a..=b).collect::<Vec<_>>().iter().filter(|n| is_valid_2(**n)).sum::<u64>())
        .collect::<Vec<_>>()
        .into_iter()
        .sum()
}