fn main() {
    let lines = include_str!("input01.txt")
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse::<u32>())
                .filter(|s| s.is_ok())
                .map(|s| s.unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let result = (0..lines[0].len())
        .map(|i| {
            (1..lines[0][i])
                .map(|e| e * (lines[0][i] - e))
                .filter(|&e| e > lines[1][i])
                .count()
        })
        .fold(1, |acc, i| acc * i);

    println!("result = {}", result);
}
