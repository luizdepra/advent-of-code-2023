fn extract_numbers(line: &str) -> Vec<u64> {
    line.split(' ')
        .map(|s| s.parse::<u64>())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect::<Vec<u64>>()
}

fn main() {
    let lines = include_str!("input01.txt")
        .split('\n')
        .collect::<Vec<&str>>();

    let mut mapped_numbers = extract_numbers(lines[0])
        .iter()
        .map(|&v| (v, false))
        .collect::<Vec<(u64, bool)>>();

    let mut index = 1;
    while index < lines.len() {
        if lines[index].contains("map:") {
            index += 1;
            while lines[index].len() > 0 {
                let nums = extract_numbers(lines[index]);
                let start = nums[1];
                let end = nums[1] + nums[2];

                for v in mapped_numbers
                    .iter_mut()
                    .filter(|(n, _)| *n >= start && *n < end)
                {
                    if v.1 {
                        continue;
                    }
                    v.0 = nums[0] + v.0 - start;
                    v.1 = true;
                }

                index += 1;
            }

            for x in mapped_numbers.iter_mut() {
                x.1 = false;
            }
        }
        index += 1;
    }

    let min_location = mapped_numbers.iter().min().unwrap();
    println!("min_location = {}", min_location.0);
}
