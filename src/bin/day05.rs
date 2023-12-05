use std::io::BufRead;

/// https://adventofcode.com/2023/day/5

#[derive(Debug)]
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

#[derive(Debug)]
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
}

fn solution_a(seeds: &[i64], sections: &[Section]) -> i64 {
    eprintln!("seeds: {:?}", seeds);

    let buffer = Vec::from(seeds);

    // TODO: Fold!
    //
    //
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

fn _solution_b(_data: &[String]) -> u32 {
    0
}

fn main() {
    let stdin = std::io::stdin();
    let mut buffer: Vec<u8> = Vec::new();

    stdin.lock()
        .read_until(0, &mut buffer).unwrap();

    let data: String = String::from_utf8(buffer).unwrap();

    let mut chunks = data.split("\n\n");
    
    let mut seeds: Vec<_> = chunks.next().unwrap().split_whitespace().skip(1)
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();
    //seeds.sort();

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
    //println!("B: {}", solution_b(&data));
}

#[cfg(test)]
mod tests {
    //use super::*;

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
