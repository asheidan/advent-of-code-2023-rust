use std::io::BufRead;

use regex::Regex;

fn solution_a(data: &str) -> u32 {
    let line_width = data.find('\n').unwrap() + 1;

    let number_pattern = Regex::new(r"[0-9]+").unwrap();
    let symbol_pattern = Regex::new("[^0-9.\n]").unwrap();


    number_pattern.find_iter(data)
        //.map(|m| { eprintln!("m: {:?}", m); m })
        .map(|m| {
            let number = m.as_str().parse::<u32>().unwrap();

            let start = m.start().saturating_sub(1);
            let end = usize::min(m.end() + 1, data.len());

            let is_part = [
                &data[start.saturating_sub(line_width)..end.saturating_sub(line_width)],
                &data[start..end],
                &data[
                    usize::min(start.saturating_add(line_width), data.len())
                        ..
                    usize::min(end.saturating_add(line_width), data.len())
                ],
            ].iter()
                .any(|&s|symbol_pattern.is_match(s));

            if is_part {
                number
            } else {
                0
            }
        })
    .sum()
}

fn solution_b(data: &str) -> u32 {
    let line_width = data.find('\n').unwrap() + 1;

    let number_pattern = Regex::new(r"[0-9]+").unwrap();

    let number_matches = data.lines()
        .map(|line| number_pattern.find_iter(line).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    data.match_indices('*')
        .map(|(index, _s)| {
            // Gear location
            let y = index / line_width;
            let x = index % line_width;

            eprintln!("*:  {}", x);

            // Get numbers on lines next to gear
            // TODO: Seems the wrong lines are considered for tha matching
            let adjacent_numbers: Vec<_> = number_matches[y.saturating_sub(1)..y.saturating_add(2)].iter()
                .flatten()
                .filter(|&m| {
                    let start = m.start();
                    let end = m.end();

                    eprint!(" : {}, {} | {}\t|", start, end, m.as_str());

                    let result = (start <= x && end >= x)  // Numbers spans gear location
                        ||
                    (end == x || start == x+1);

                    eprintln!(" {}", result);

                    result
                })
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .collect();

            match adjacent_numbers[..] {
                [a, b] => a * b,
                _ => 0,
            }

        })
        .sum()
}

fn main() {
    let stdin = std::io::stdin();
    let mut buffer = Vec::new();
    stdin.lock()
        .read_until(0, &mut buffer).unwrap();

    let data  = String::from_utf8(buffer).unwrap();

    let _foo = &data[..];

    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn sat_sub() {
        let result = 1_usize.saturating_sub(200);

        assert_eq!(0, result);
    }

    mod foo {
        //use super::*;

        macro_rules! from_tests {
            ($($name:ident: $value: expr,)*) => {
            $(
                #[test]
                fn $name() {
                    // Given
                    let (input, expected) = $value;
                    let input = String::from(input);

                    // When
                    let result = input.parse::<u32>().unwrap();

                    // Then
                    assert_eq!(expected, result);
                }
            )*
            }
        }

        from_tests! {
            one_1: ("1", 1),
        }
    }
}
