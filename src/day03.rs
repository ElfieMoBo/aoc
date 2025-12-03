pub fn get_part_1() -> u64 {
    357
}

fn two_max(list: Vec<u64>) -> u64 {
    let slice = list[..list.len()-1].to_vec();
    let max = slice.iter().enumerate().rev().max_by_key(|&(_idx, &val)| val).expect("Failed to get max");
    let slice = &list[(max.0 + 1)..];
    let submax = slice.iter().max().expect("Failed to get sub-max");
    *max.1 * 10 + submax
}

pub fn part_1(file: &String) -> u64 {
    file.lines()
        .into_iter()
        .map(|line| line.chars()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|char| {char.to_string().parse::<u64>().unwrap()})
                .collect::<Vec<_>>())
                .collect::<Vec<_>>()
                .into_iter()
                .map(|vec| two_max(vec))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .sum()
}

pub fn get_part_2() -> u64 {
    3121910778619
}

fn twelve_max(list: Vec<u64>) -> u64 {
    let mut res = 0;
    let mut slice = list[..list.len()-11].to_vec();
    let mut previous : Option<usize> = None;
    for i in (0..12).rev() {
        let prev = match previous {
            None => 0,
            Some(prev) => {
                slice = list[(prev)..=list.len() - (i+1)].to_vec();
                prev
            },
        };
        let max = slice.iter().enumerate().rev().max_by_key(|&(_idx, &val)| val).expect("Failed to get max");
        previous = Some(prev+1 + max.0);
        res += max.1 * u64::pow(10, i as u32);
    }
    res
}

pub fn part_2(file: &String) -> u64 {
    file.lines()
        .into_iter()
        .map(|line| line.chars()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|char| {char.to_string().parse::<u64>().unwrap()})
            .collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|vec| twelve_max(vec))
        .collect::<Vec<_>>()
        .into_iter()
        .sum()
}