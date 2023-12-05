use std::io::BufRead;
use std::cmp::PartialEq;
use std::ops::Range;

/// https://adventofcode.com/2023/day/5

#[derive(Debug, PartialEq, Eq)]
struct Mapping {
    source: i64,
    target: i64,
    length: i64,
}

impl Mapping {
    fn convert(&self, number: &i64) -> i64 {
        // NOTE: This does not check bounds
        number + (self.target - self.source)
    }

    fn convert_range(&self, range: &Range<i64>) -> Vec<Range<i64>> {
        let self_end = self.source + self.length;
        let self_offset = self.target - self.source;


        if self_end <= range.start || self.source >= range.end {
            vec!(range.to_owned())
        }
        else if self.contains(&range.start) {
            if self.contains(&range.end) {  // Range is contained
                vec!((range.start + self_offset)..(range.end + self_offset))
            }
            else if self_end < range.end {  // Covered lowered half of range
                vec!((range.start + self_offset)..(self_end + self_offset), self_end..range.end)
            }
            else {
                vec!()
            }
        }
        else if self.contains(&range.end) {  // Covered upper half of range
            vec!(range.start..self.source, (self.source + self_offset)..(range.end + self_offset))
        }
        else {
            vec!()
        }
    }

    fn contains(&self, item: &i64) -> bool {
        (self.source..(self.source + self.length)).contains(item)
    }
}

impl From<&str> for Mapping {
    fn from(value: &str) -> Self {
        let numbers: Vec<_> = value.split_whitespace()
            .map(|number| number.parse::<i64>().unwrap())
            .collect();

        match numbers[..] {
            [target, source, length] => Self { source, target, length },
            _ => panic!("Don't know how to create Mapping from: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Section {
    mappings: Vec<Mapping>,
}

impl Section {
    fn convert(&self, number: &i64) -> i64 {
        let mapping: &Mapping = self.mappings.iter()
            .find(|&mapping| mapping.contains(number))
            .unwrap_or(&Mapping { source: 0, target: 0, length: 0 });

        mapping.convert(number)
    }

    fn convert_range(&self, range: &Range<i64>) -> Vec<Range<i64>> {
        self.mappings.iter()
            .fold(vec!(range.to_owned()), |input, mapping| {
                input.iter()
                    .map(|input_range| mapping.convert_range(input_range))
                    .flatten()
                    .collect()
            })
    }
}

impl From<&str> for Section {
    fn from(value: &str) -> Self {
        let mut mappings: Vec<Mapping> = value.lines().skip(1)
            .map(Mapping::from)
            .collect();

        mappings.sort_by_key(|mapping| (mapping.source, mapping.length));

        Section { mappings }
    }
}

fn solution_a(seeds: &[i64], sections: &[Section]) -> i64 {
    eprintln!("seeds: {:?}", seeds);

    let buffer = Vec::from(seeds);

    let result = sections.iter()
        .fold(buffer, |input, section| {
            let result = input.iter()
                .map(|number| section.convert(number))
                .collect::<Vec<i64>>();

            //eprintln!("  : {:?}", result);

            result
        });

    *result.iter().min().unwrap()
}

fn solution_b(_seeds: &[i64], _sections: &[Section]) -> i64 {
    0
}

fn main() {
    let stdin = std::io::stdin();
    let mut buffer: Vec<u8> = Vec::new();

    stdin.lock()
        .read_until(0, &mut buffer).unwrap();

    let data: String = String::from_utf8(buffer).unwrap();

    let mut chunks = data.split("\n\n");
    
    let seeds: Vec<_> = chunks.next().unwrap().split_whitespace().skip(1)
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();

    let sections: Vec<Section> = chunks
        .map(|chunk| {
            let mut mappings: Vec<Mapping> = chunk.lines().skip(1)
                .map(Mapping::from)
                .collect();

            mappings.sort_by_key(|mapping| (mapping.source, mapping.length));

            Section { mappings }
        })
        .collect();


    println!("A: {}", solution_a(&seeds, &sections));
    println!("B: {}", solution_b(&seeds, &sections));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod mapping {
        use super::*;
        mod convert_range {
            use super::*;

            #[test]
            fn should_convert_range_below_mapping_to_vec() {
                // Given
                let mapping = Mapping { source: 100, target: 100, length: 1 };
                let input = 1..14;

                // When
                let result = mapping.convert_range(&input);

                // Then
                let expected = vec!(1..14);
                assert_eq!(expected, result);
            }

            #[test]
            fn should_convert_range_above_mapping_to_vec() {
                // Given
                let mapping = Mapping { source: 1, target: 100, length: 10 };
                let input = 20..42;

                // When
                let result = mapping.convert_range(&input);

                // Then
                let expected = vec!(20..42);
                assert_eq!(expected, result);
            }

            #[test]
            fn should_shift_contained_range_to_vec() {
                // Given
                let mapping = Mapping { source: 1, target: 101, length: 15 };
                let input = 1..14;

                // When
                let result = mapping.convert_range(&input);

                // Then
                let expected = vec!(101..114);
                assert_eq!(expected, result);
            }

            #[test]
            fn should_shift_covered_lower_half_range() {
                // Given
                let mapping = Mapping { source: 20, target: 40, length: 10 };
                let input = 25..35;

                // When
                let result = mapping.convert_range(&input);

                // Then
                let expected = vec!(45..50, 30..35);
                assert_eq!(expected, result);
            }

            #[test]
            fn should_shift_covered_upper_half_range() {
                // Given
                let mapping = Mapping { source: 20, target: 40, length: 10 };
                let input = 15..25;

                // When
                let result = mapping.convert_range(&input);

                // Then
                let expected = vec!(15..20, 40..45);
                assert_eq!(expected, result);
            }
            // TODO: Covering middle
        }
    }
    mod section {
        use super::*;

        mod from {
            use super::*;

            #[test]
            fn name_only_should_result_in_no_mappings() {
                // Given
                let input = "foo-to-bar map:";

                // When
                let result = Section::from(input);

                // Then
                let expected = Section { mappings: Vec::new() };
                assert_eq!(expected, result);
            }

            #[test]
            fn single_mapping_should_result_in_one_mappings() {
                // Given
                let input = "foo-to-bar map:\n1 0 15";

                // When
                let result = Section::from(input);

                // Then
                let expected = Section { mappings: vec!(Mapping { source: 0, target: 1, length: 15 }) };
                assert_eq!(expected, result);
            }
        }

        mod convert_range {
            use super::*;

            mod empty_section {
                use super::*;

                #[test]
                fn should_convert_range_to_vec() {
                    // Given
                    let section = Section::from("foo-to-bar map:");
                    let input = 1..14;

                    // When
                    let result = section.convert_range(&input);

                    // Then
                    let expected = vec!(1..14);
                    assert_eq!(expected, result);
                }
            }
            mod single_mapping_section {
                use super::*;

                #[test]
                fn should_convert_free_range_to_vec() {
                    // Given
                    let section = Section::from("foo-to-bar map:100 20 15");
                    let input = 1..14;

                    // When
                    let result = section.convert_range(&input);

                    // Then
                    let expected = vec!(1..14);
                    assert_eq!(expected, result);
                }

                #[test]
                fn should_shift_contained_range_to_vec() {
                    // Given
                    let section = Section::from("foo-to-bar map:101 1 15");
                    let input = 1..14;

                    // When
                    let result = section.convert_range(&input);

                    // Then
                    let expected = vec!(101..114);
                    assert_eq!(expected, result);
                }
            }
        }
    }

    /*
    #[test]
    fn foo() {
        // Given
        let input = "1";

        // When
        let result = input.parse::<u32>().unwrap();

        // Then
        assert_eq!(1, result);
    }
    */
}
