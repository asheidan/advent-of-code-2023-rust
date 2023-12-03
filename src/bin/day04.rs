use std::io::BufRead;

fn solution_a(_data: &str) -> u32 {
    0
}

fn solution_b(_data: &str) -> u32 {
    0
}

fn main() {
    let stdin = std::io::stdin();
    let mut buffer = Vec::new();
    stdin.lock()
        //.lines()
        .read_until(0, &mut buffer).unwrap();

    let data  = String::from_utf8(buffer).unwrap();

    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}

#[cfg(test)]
mod tests {
    //use super::*;
}
