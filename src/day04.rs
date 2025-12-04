fn parse(file: &String) -> Vec<Vec<String>> {
    file.lines()
        .into_iter()
        .map(|line|
            line.chars()
                .collect::<Vec<char>>().
                into_iter()
                .map(|char| { char.to_string() })
                .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn get_unchecked(characters: &Vec<Vec<String>>, i: usize, j: usize) -> Vec<(i64, i64)> {
    let n = characters.len();
    let m = characters[0].len();
    let mut unchecked = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    if i==0 {
        unchecked = unchecked.iter().filter(|c| {**c!=(-1, -1) && **c!=(-1, 0) && **c!=(-1,1)}).map(|c| *c).collect::<Vec<_>>();
    } else if i == m - 1 {
        unchecked = unchecked.iter().filter(|c| {**c!=(1, -1) && **c!=(1, 0) && **c!=(1,1)}).map(|c| *c).collect::<Vec<_>>();
    }
    if j==0 {
        unchecked = unchecked.iter().filter(|c| {**c!=(-1, -1) && **c!=(0, -1) && **c!=(1,-1)}).map(|c| *c).collect::<Vec<_>>();
    } else if j == n - 1 {
        unchecked = unchecked.iter().filter(|c| {**c!=(-1, 1) && **c!=(0, 1) && **c!=(1,1)}).map(|c| *c).collect::<Vec<_>>();
    }
    unchecked
}

fn is_accessible(characters: &Vec<Vec<String>>, i: usize, j: usize) -> bool {
    let mut count = 0;
    let unchecked = get_unchecked(characters, i, j);
    for (index, jindex) in unchecked.iter() {
        if characters[(i as i64 + *index) as usize][(j as i64 + *jindex) as usize].contains("@") {
            count += 1;
        }
    }
    count < 4
}

fn count_accessible(characters: &mut Vec<Vec<String>>, change: bool) -> u64 {
    let mut count = 0;
    for i in 0..characters[0].len() {
        for j in 0..characters.len() {
            if characters[i][j].contains(".") {
                continue;
            }
            if is_accessible(characters, i, j) {
                if change {
                    characters[i][j] = '.'.to_string();
                }
                count += 1;
            }
        }
    }
    count
}

pub fn get_part_1() -> u64 {
    13
}

pub fn part_1(file: &String) -> u64 {
    let mut characters = parse(file);
    count_accessible(&mut characters, false)
}

pub fn get_part_2() -> u64 {
    43
}

pub fn part_2(file: &String) -> u64 {
    let mut characters = parse(file);
    let mut new = count_accessible(&mut characters, true);
    let mut count = new;
    while new != 0 {
        new = count_accessible(&mut characters, true);
        count += new;
    }
    count
}