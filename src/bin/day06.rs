use std::io::BufRead;

/// https://adventofcode.com/2023/day/6


fn number_of_wins(time: i64, distance: i64) -> i64 {
    let half_time = time as f64 / 2.0;
    let root_portion = f64::sqrt(f64::powi(half_time, 2) - (distance as f64));

    eprint!(" | {} | {}", half_time, root_portion);

    let start_search = (half_time - root_portion).floor() as i64;
    let end_search = (half_time + root_portion).floor() as i64;

    eprint!(" | s:{} | :e{}", start_search, end_search);

    let start = (start_search..(start_search + 5))
        .find(|t| (time - t) * t > distance)
        .unwrap();
    let end = (end_search..(end_search + 5))
        .find(|t| (time - t) * t <= distance)
        .unwrap();

    eprintln!(" | {} - {}", end, start);

    end - start

    /*
    (0..time)
        .filter(|t| (time - t) * t > distance)
        .count() as i64
    */
}


fn solution_a(data: &[Vec<i64>]) -> i64 {
    if let [times, distances] = &data[..] {
        std::iter::zip(times, distances)
            .map(|(&time, &distance)| {
                number_of_wins(time, distance)
            })
            .reduce(|a, b| a * b)
            .unwrap()
   }
    else {
        panic!("Wrong number of vectors");
    }
}

fn solution_b(_data: &[Vec<i64>]) -> i64 {
    let time = 44899691;
    let distance = 277113618901768;

    number_of_wins(time, distance)
}

fn main() {
    let stdin = std::io::stdin();

    let data: Vec<Vec<_>> = stdin.lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.split_whitespace().skip(1).map(|n| n.parse::<i64>().unwrap()).collect())
        .collect();


    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // Given
        let time = 7;
        let distance = 9;

        let result = number_of_wins(time, distance);

        let expected = 4;
        assert_eq!(expected, result);
    }

    #[test]
    fn example_2() {
        // Given
        let time = 15;
        let distance = 40;

        let result = number_of_wins(time, distance);

        let expected = 8;
        assert_eq!(expected, result);
    }

    #[test]
    fn example_3() {
        // Given
        let time = 30;
        let distance = 200;

        let result = number_of_wins(time, distance);

        let expected = 9;
        assert_eq!(expected, result);
    }

    #[test]
    fn example_4() {
        // Given
        let time = 71530;
        let distance = 940200;

        let result = number_of_wins(time, distance);

        let expected = 71503;
        assert_eq!(expected, result);
    }
}
