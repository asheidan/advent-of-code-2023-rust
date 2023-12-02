use std::io::BufRead;

fn solution_a(_data: &[String]) -> u32 {
    0
}
fn solution_b(_data: &[String]) -> u32 {
    0
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<String> = stdin.lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}

#[cfg(test)]
mod tests {
    //use super::*;

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
