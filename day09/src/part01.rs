fn calc_next(numbers: &Vec<i64>) -> i64 {
    if numbers.iter().all(|n| *n == 0) {
        return 0;
    }

    numbers.last().unwrap()
        + calc_next(
            &numbers
                .windows(2)
                .map(|n| n[1] - n[0])
                .collect::<Vec<i64>>(),
        )
}

fn main() {
    let result = include_str!("input01.txt")
        .lines()
        .map(|l| {
            calc_next(
                &l.split(' ')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .sum::<i64>();

    println!("result = {}", result);
}
