use std::io::BufRead;

#[derive(Debug, PartialEq)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let [number_str, color, ..] = value.split(" ").collect::<Vec<_>>()[..] else { todo!() };
        let number = number_str.parse::<u32>().unwrap();

        match color {
            "red" => Self::Red(number),
            "green" => Self::Green(number),
            "blue" => Self::Blue(number),
            _ => panic!("Whooo! Panic!!!!"),
        }
    }
}

impl From<&String> for Cube {
    fn from(value: &String) -> Self {
        Self::from(&value[..])
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    number: u32,
    handfuls: Vec<Vec<Cube>>,
}

impl From<&String> for Game {
    fn from(value: &String) -> Self {
        let [name, data, ..] = value.split(": ").collect::<Vec<_>>()[..] else { todo!() };

        let number = name[5..].parse::<u32>().unwrap();

        let handfuls = data.split("; ")
            .map(|hand_description| hand_description.split(", ")
                 .map(|cube| Cube::from(cube))
                 .collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { number, handfuls }
    }
}

fn solution_a(data: &[String]) -> u32 {
    data.iter()
        .map(Game::from)
        .map(|game| {  // Number of cubes shown per game
            let (red, green, blue) = game.handfuls.iter()
             .flatten()
             .fold((0, 0, 0), |result, cube| match (result, cube) {
                 ((r, g, b), Cube::Red(n)) => (r + n, g, b),
                 ((r, g, b), Cube::Green(n)) => (r, g + n, b),
                 ((r, g, b), Cube::Blue(n)) => (r, g, b + n),
             });

            if red <= 12 && green <= 13 && blue <= 14 {
                game.number
            }
            else {
                0
            }
        })
    .sum()
}
fn solution_b(_data: &[String]) -> u32 {
    0
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();

    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod cube {
        use super::*;

        macro_rules! from_tests {
            ($($name:ident: $value: expr,)*) => {
            $(
                #[test]
                fn $name() {
                    // Given
                    let (input, expected) = $value;
                    let input = String::from(input);

                    // When
                    let result = Cube::from(&input);

                    // Then
                    assert_eq!(expected, result);
                }
            )*
            }
        }

        from_tests! {
            blue_4: ("4 blue", Cube::Blue(4)),
            green_42: ("42 green", Cube::Green(42)),
            red_5: ("5 red", Cube::Red(5)),
        }
    }
}
