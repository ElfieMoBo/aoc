fn parse_line(line: &str) -> Vec<&str> {
    line.split(',').collect::<Vec<&str>>()
}

pub fn part_1(file: &String) -> u64 {
    // Add numbers that are composed of some sequence of digits repeated twice
    let mut counter = 0;
    let ranges = parse_line(file);
    for range in ranges {
        let vector = range.split('-').collect::<Vec<&str>>();
        let (inf, sup): (u64, u64) = (
            vector[0].parse().expect("failed to parse inf"),
            vector[1].parse().expect("failed to parse sup"),
        );
        for i in inf..=sup {
            let i_string = i.to_string();
            let mut cut = i_string.len();
            if (cut % 2) == 1 {
                continue;
            } else {
                cut = cut / 2
            }
            let (low_part, high_part): (u64, u64) = (
                i_string[..cut].parse().expect("failed to parse"),
                i_string[cut..].parse().expect("failed to parse"),
            );
            if low_part == high_part {
                counter += i;
            }
        }
    }
    counter
}

pub fn part_2(file: &String) -> u64 {
    // Add numbers that are composed of some sequence of digits repeated at least twice
    let mut counter = 0;
    let ranges = parse_line(file);
    for range in ranges {
        let vector = range.split('-').collect::<Vec<&str>>();
        let (inf, sup): (u64, u64) = (
            vector[0].parse().expect("failed to parse inf"),
            vector[1].parse().expect("failed to parse sup"),
        );
        for i in inf..=sup {
            let i_string = i.to_string();
            let cut = i_string.len();
            if cut == 1 {
                continue;
            }
            let mut divisors = divisors::get_divisors(cut);
            if !divisors.contains(&cut) {
                divisors.push(cut);
            }
            for div in divisors.iter() {
                let mut slices = Vec::new();
                let n = cut / div;
                for j in 0..(*div) {
                    let slice = i_string[j * n..(j + 1) * n].to_string();
                    slices.push(slice);
                }
                if slices.iter().min().unwrap() == slices.iter().max().unwrap() {
                    counter += i;
                    break;
                }
            }
        }
    }
    counter
}
