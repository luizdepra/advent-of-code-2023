fn main() {
    let values = include_str!("input01.txt")
        .lines()
        .map(|l| {
            let numbers = l.split(':').skip(1).collect::<Vec<&str>>();
            numbers
                .first()
                .unwrap()
                .chars()
                .filter(|c| c.is_numeric())
                .fold(0, |acc, v| acc * 10 + v.to_digit(10).unwrap() as u64)
        })
        .collect::<Vec<u64>>();

    let mut start = 0;
    for i in 1..values[0] {
        let distance = i * (values[0] - i);
        if distance > values[1] {
            break;
        }
        start = i;
    }

    let mut end = 0;
    for i in (start..(values[0] - 1)).rev() {
        let distance = i * (values[0] - i);
        if distance > values[1] {
            break;
        }
        end = i;
    }

    println!("result = {}", end - start - 1);
}
