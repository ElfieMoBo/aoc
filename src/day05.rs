use std::cmp::{max, min};

fn parse(file: &String) -> (Vec<(u64, u64)>, Vec<u64>) {
    let range_id = file.split("\n\n").collect::<Vec<&str>>();
    let ranges = range_id[0].trim().split("\n").map(|range| {
        let mut split = range.split("-").collect::<Vec<&str>>();
        (split[0].parse().expect("cannot parse number"), split[1].parse().expect("cannot parse number"))
    }).collect::<Vec<(u64, u64)>>();
    let ids = range_id[1].trim().split("\n").map(|id| id.parse().expect("cannot parse number")).collect::<Vec<u64>>();
    (ranges, ids)
}

fn is_in_range(range: (u64, u64), n: u64) -> bool {
    if n >= range.0 && n <= range.1 {
        return true;
    }
    false
}

fn is_in_ranges(ranges: &Vec<(u64, u64)>, n: u64) -> bool {
    let count = ranges.iter()
        .filter(|couple| is_in_range(**couple, n) )
        .count();
    count > 0
}

fn count_in_range(ranges: Vec<(u64, u64)>, ids: Vec<u64>) -> u64 {
    ids.into_iter()
        .filter(|id| is_in_ranges(&ranges, *id))
        .count() as u64
}

pub fn get_part_1() -> u64 {
    3
}

pub fn part_1(file: &String) -> u64 {
    let (range, ids) = parse(file);
    count_in_range(range, ids)
}

pub fn get_part_2() -> u64 {
    14
}

fn regroup_two_ranges(range_a: (u64, u64), range_b: (u64, u64)) -> ((u64, u64), bool) {
    if range_b.0 <= range_a.1 && range_a.0 <= range_b.1 {
        return ((min(range_a.0, range_b.0), max(range_a.1, range_b.1)), true);
    }
    (range_a, false)
}

fn regroup_ranges(ranges: &mut Vec<(u64, u64)>, range: (u64, u64)) -> Vec<(u64, u64)> {
    let mut res = ranges.clone();
    let mut to_add = true;
    ranges
        .iter()
        .map(|range_a| { regroup_two_ranges(*range_a, range) })
        .enumerate()
        .for_each(|(idx, content)| {
            if content.1 {
                to_add = false;
                res[idx] = content.0;
            }
        });
    if to_add {
        res.push(range);
    }
    res
}

fn restrict_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut without = vec![ranges[0].clone()];
    ranges.iter()
        .for_each(|(inf, sup) | {
            without = regroup_ranges(&mut without, (*inf, *sup));
        });
    without
}

pub fn part_2(file: &String) -> u64 {
    let (mut ranges, _) = parse(file);
    let mut previous = ranges.clone();
    let mut next = restrict_ranges(&mut ranges);
    while next != previous {
        previous = next.clone();
        next = restrict_ranges(&mut next);
    }
    next.iter().map(|(inf, sup)| sup-inf +1).sum()
}