use std::io::BufRead;

fn solution_a(data: &[String]) -> i32 {
    data.iter()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>())
        .map(|numbervector: Vec<char>| {
            [numbervector.first(), numbervector.last()]
                .into_iter()
                .map(|c| c.unwrap_or(&'0'))
                .collect::<String>()
        })
        .map(|number_str| number_str.parse::<i32>().unwrap())
        .sum()
}

fn solution_b(data: &[String]) -> u32 {
    let names = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    data.iter()
        .map(|line| {
            println!("> {}", line);
            line
        })
        .map(|line| {
            print!("< ");
            //let mut mut_line= String::from(line);
            let line_len = line.len();
            let result = (0..line_len)
                .filter_map(|start| {
                    let chunk = &line[start..];
                    // eprintln!("  c: {}", chunk);

                    let first_char = chunk.chars().next().unwrap();

                    let result = match first_char.is_digit(10) {
                        true => Some(first_char.to_digit(10).unwrap()),
                        false => names
                            .iter()
                            .enumerate()
                            .filter_map(|(index, name)| {
                                if chunk.starts_with(name) {
                                    Some(index as u32 + 1)
                                } else {
                                    None
                                }
                            })
                            .next(),
                    };

                    print!(
                        "{}",
                        match result {
                            None => ' ',
                            Some(number) => char::from_u32(number + ('0' as u32)).unwrap(),
                        }
                    );

                    result
                })
                .collect::<Vec<u32>>();

            println!();

            result
        })
        .map(|numbers| 10 * numbers.first().unwrap() + numbers.last().unwrap())
        .map(|line| {
            println!("= {}\n", line);
            line
        })
        .sum()
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();

    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}
