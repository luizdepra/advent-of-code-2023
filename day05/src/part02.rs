#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct NumberRange(i64, i64);

impl NumberRange {
    fn intersects_with(&self, other: &NumberRange) -> bool {
        (self.0 >= other.0 && self.0 < other.1) || (self.1 >= other.0 && self.1 < other.1)
    }

    fn intersection(&self, other: &NumberRange) -> NumberRange {
        NumberRange(i64::max(self.0, other.0), i64::min(self.1, other.1))
    }

    fn split_ranges(&self, other: &NumberRange) -> Vec<NumberRange> {
        let mut result = vec![];

        let intersection = self.intersection(other);
        result.push(intersection.clone());

        if self.0 < other.0 {
            result.push(NumberRange(self.0, intersection.0));
        }
        if self.1 > other.1 {
            result.push(NumberRange(intersection.1, self.1));
        }

        result
    }
}

#[derive(Debug)]
struct MappingRange(i64, i64, i64, i64);

impl MappingRange {
    fn source_range(&self) -> NumberRange {
        NumberRange(self.0, self.1)
    }

    fn destination_range(&self) -> NumberRange {
        NumberRange(self.2, self.3)
    }
}

fn extract_numbers(line: &str) -> Vec<i64> {
    line.split(' ')
        .map(|s| s.parse::<i64>())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect::<Vec<i64>>()
}

fn main() {
    let lines = include_str!("input01.txt")
        .split('\n')
        .collect::<Vec<&str>>();

    let mut numbers = extract_numbers(lines[0])
        .chunks(2)
        .map(|v| (NumberRange(v[0], v[0] + v[1]), false))
        .collect::<Vec<(NumberRange, bool)>>();

    let mut index = 1;
    let mut mapping_list = vec![];
    while index < lines.len() {
        if lines[index].contains("map:") {
            index += 1;

            let mut mapping = vec![];
            while lines[index].len() > 0 {
                let nums = extract_numbers(lines[index]);

                mapping.push(MappingRange(
                    nums[1],
                    nums[1] + nums[2],
                    nums[0],
                    nums[0] + nums[2],
                ));

                index += 1;
            }
            mapping_list.push(mapping);
        }
        index += 1;
    }

    for mapping in mapping_list {
        for mapping_item in mapping {
            let case_list = numbers
                .iter()
                .cloned()
                .enumerate()
                .filter(|(_, n)| !n.1 && n.0.intersects_with(&mapping_item.source_range()))
                .collect::<Vec<(usize, (NumberRange, bool))>>();
            for (index, case) in case_list {
                let source_range = mapping_item.source_range();
                let splits = case.0.split_ranges(&source_range);

                let new_range = splits[0];
                let dest_range = mapping_item.destination_range();
                let diff = dest_range.0 - source_range.0;

                (numbers[index].0).0 = new_range.0 + diff;
                (numbers[index].0).1 = new_range.1 + diff;
                numbers[index].1 = true;

                numbers.extend(splits.iter().skip(1).map(|n| (*n, false)));
            }
        }

        for item in numbers.iter_mut() {
            item.1 = false;
        }
    }

    let min_location = numbers.iter().map(|n| n.0).min().unwrap();
    println!("min_location = {}", min_location.0);
}
