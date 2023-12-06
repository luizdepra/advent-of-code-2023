#[derive(Debug, Clone, Copy)]
struct NumberRange(u64, u64);

impl NumberRange {
    fn intersects_with(&self, other: &NumberRange) -> bool {
        (self.0 >= other.0 && self.0 < other.1) || (self.1 >= other.0 && self.1 < other.1)
    }

    fn intersection(&self, other: &NumberRange) -> NumberRange {
        NumberRange(u64::max(self.0, other.0), u64::min(self.1, other.1))
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
struct MappingRange(u64, u64, u64, u64);

impl MappingRange {
    fn source_range(&self) -> NumberRange {
        NumberRange(self.0, self.1)
    }

    fn destination_range(&self) -> NumberRange {
        NumberRange(self.2, self.3)
    }
}

fn extract_numbers(line: &str) -> Vec<u64> {
    line.split(' ')
        .map(|s| s.parse::<u64>())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect::<Vec<u64>>()
}

fn main() {
    let lines = include_str!("inputtest.txt")
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

    println!("mapping_list = {:?}", mapping_list);
    println!("numbers = {:?}", numbers);

    for mapping in mapping_list {
        println!("  mapping = {:?}", mapping);
        for mapping_item in mapping {
            println!("    mapping_item = {:?}", mapping_item);
            let case_list = numbers
                .iter()
                .cloned()
                .enumerate()
                .filter(|(_, n)| n.0.intersects_with(&mapping_item.source_range()))
                .collect::<Vec<(usize, (NumberRange, bool))>>();
            for (index, case) in case_list {
                println!("      case = {:?}", case);

                let splits = case.0.split_ranges(&mapping_item.source_range());

                let range = splits[0].1 - splits[0].1;
                let dest_range = mapping_item.destination_range();
                // traduzir src to dst. provavelmente precisa usar a intersection atualizada
                //(numbers[index].0).0 = dest_range.0
                //numbers[index].1 = true;

                println!("      number_updated = {:?}", numbers[index]);
                println!("      splits = {:?}", splits);

                numbers.extend(splits.iter().skip(1).map(|n| (*n, false)));
            }
        }

        println!("numbers = {:?}", numbers);
        break;
    }

    let min_location = numbers.first().unwrap();
    println!("min_location = {}", (min_location.0).0);
}
