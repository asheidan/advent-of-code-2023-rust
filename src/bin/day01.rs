use std::io::BufRead;

fn solution_a(data: &[i32]) -> i32 {
    data.iter().sum()
}

fn solution_b(_data: &[i32]) -> i32 {
    0
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<i32> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line: String| line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>())
        .map(|numbervector: Vec<char>| 
             [numbervector.first().unwrap(), numbervector.last().unwrap()].into_iter().collect::<String>()
         )
        .map(|number_str| number_str.parse::<i32>().unwrap())

        //.split_by_none().into_iter()
        .collect();

    //data.sort_by_key(|&value| std::cmp::Reverse(value));
    //
    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}
