use std::ops::RangeInclusive;

fn bounded_inclusive_range(start: usize, end: usize, max: usize) -> RangeInclusive<usize> {
    let mut range_start = start as i32 - 1;
    let mut range_end = end as i32 + 1;

    if range_start < 0 {
        range_start = start as i32;
    }
    if range_end >= max as i32 {
        range_end = end as i32;
    }

    (range_start as usize)..=(range_end as usize)
}

fn check_for_neighbor_symbols(
    matrix: &Vec<Vec<char>>,
    x_start: usize,
    x_end: usize,
    y: usize,
) -> bool {
    for j in bounded_inclusive_range(y, y, matrix.len()) {
        for i in bounded_inclusive_range(x_start, x_end, matrix[j].len()) {
            if i >= x_start && i <= x_end && j == y {
                continue;
            } else if !matrix[j][i].is_numeric() && matrix[j][i] != '.' {
                return true;
            }
        }
    }

    false
}

fn main() {
    let input = include_str!("input01.txt");

    let matrix = input
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    let mut x = 0;
    let mut y = 0;
    while y < matrix.len() {
        while x < matrix[y].len() {
            if matrix[y][x].is_numeric() {
                let mut num = 0;
                let mut step = 0;
                for i in x..matrix[y].len() {
                    if !matrix[y][i].is_numeric() {
                        break;
                    }
                    num = num * 10 + matrix[y][i].to_digit(10).unwrap();
                    step += 1;
                }

                if check_for_neighbor_symbols(&matrix, x, x + step - 1, y) {
                    sum += num;
                }

                x += step - 1;
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

    println!("Sum = {}", sum);
}
