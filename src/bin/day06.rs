use std::io::BufRead;

/// https://adventofcode.com/2023/day/6


fn number_of_wins(time: i64, distance: i64) -> i64 {
    // Prepare for aproximate solve
    let half_time = time as f64 / 2.0;
    let root_portion = f64::sqrt(f64::powi(half_time, 2) - (distance as f64));

    // Find approximate roots, aim for low value
    let start_search = (half_time - root_portion).floor() as i64;
    let end_search = (half_time + root_portion).floor() as i64;

    // Find "exact" roots or limits of the interval (inclusive start, exclusive end)
    // 5 should be enough...
    let start = (start_search..(start_search + 5))
        .find(|t| (time - t) * t > distance)
        .unwrap();
    let end = (end_search..(end_search + 5))
        .find(|t| (time - t) * t <= distance)
        .unwrap();

    end - start

    /*
    // Exhaustive search
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

    macro_rules! examples {
        ($($name:ident: $value: expr,)*) => {
        $(
            #[test]
            fn $name() {
                // Given
                let (time, distance, expected) = $value;

                // When
                let result = number_of_wins(time, distance);

                // Then
                assert_eq!(expected, result);
            }
        )*
        }
    }

    examples! {
        example_1_1: (7, 9, 4),
        example_1_2: (15, 40, 8),
        example_1_3: (30, 200, 9),
        example_2_1: (71530, 940200, 71503),
    }
}
