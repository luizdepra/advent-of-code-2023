use std::ops::RangeInclusive;

fn bounded_inclusive_range(value: usize, max: usize) -> RangeInclusive<usize> {
    let signed_value = value as i32;
    let mut start = signed_value - 1;
    let mut end = signed_value + 1;

    if start < 0 {
        start = signed_value;
    }
    if end >= max as i32 {
        end = signed_value;
    }

    (start as usize)..=(end as usize)
}

fn extract_number(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut num = 0;

    let mut start = x as i32;
    for i in 0.. {
        if matrix[y][x + i as usize].is_numeric() {}
    }
}

fn get_neighbor_numbers(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<i32> {
    let mut result = vec![];

    for j in bounded_inclusive_range(y, matrix.len()) {
        for i in bounded_inclusive_range(x, matrix[j].len()) {
            if i == x && j == y {
                continue;
            } else if matrix[j][i].is_numeric() {
                result.push(extract_number(&matrix, x, y));
            }
        }
    }

    result
}

fn main() {
    let input = include_str!("input01.txt");

    let matrix = input
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] == '*' {
                let numbers = get_neighbor_numbers(&matrix, x, y);
                if numbers.len() == 2 {
                    sum += numbers[0] * numbers[1];
                }
            }
        }
    }

    println!("Sum = {}", sum);
}
