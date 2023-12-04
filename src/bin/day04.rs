use std::collections::HashSet;
use std::io::BufRead;

/// https://adventofcode.com/2023/day/4

struct Card {
    winners: HashSet<u32>,
    numbers: Vec<u32>,
}

impl From<String> for Card {
    fn from(value: String) -> Self {
        let mut data_iter = value.split(" | ");

        let winners: HashSet<u32> = data_iter.next().unwrap().split_whitespace().skip(2)
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let numbers: Vec<u32> = data_iter.next().unwrap().split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();


        Card { winners, numbers }
    }
}

fn solution_a(data: &[Card]) -> u32 {
    data.iter()
        .map(|card| {
            // Number of wins
            let number_of_wins: u32 = card.numbers.iter()
                .filter_map(|number| card.winners.contains(number).then_some(1))
                .sum();

            if 0 < number_of_wins {
                1 << number_of_wins.saturating_sub(1)
            } else {
                0
            }
        })
        .sum()
}

fn solution_b(data: &[Card]) -> u32 {
    let number_of_wins: Vec<u32> = data.iter()
        .map(|card| {
            // Number of wins
            card.numbers.iter()
                .filter_map(|number| card.winners.contains(number).then_some(1))
                .sum()
        })
        .collect();

    let mut cards: Vec<u32> = (0..number_of_wins.len()).map(|_| 1).collect();
    assert_eq!(number_of_wins.len(), cards.len());

    number_of_wins.iter().enumerate()
        .for_each(|(index, &wins)| {
            let current_count = *cards.get(index).unwrap();
                  ((index + 1)..=(index+wins as usize)).for_each(|index| {
                      //*cards.get_mut(index).unwrap() *= 2;
                      *cards.get_mut(index).unwrap() += current_count;
                  })
        });


    cards.iter().sum()
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<_> = stdin.lock()
        .lines()
        .map(|line| line.unwrap())
        .map(Card::from)
        .collect();

    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn sat_sub() {
        // Given
        let input = "1";

        // When
        let result = input.parse::<u32>().unwrap();

        // Then
        assert_eq!(1, result);
    }

    #[test]
    fn empty_range() {
        let result = (1..1).map(|_| 1).sum();
        assert_eq!(0, result);
    }
}
