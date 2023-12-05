use std::io::BufRead;

fn _solution_a(_data: &String) -> i64 {
    0
}

fn _solution_b(_data: &String) -> i64 {
    0
}

fn main() {
    let stdin = std::io::stdin();
    let mut buffer: Vec<u8> = Vec::new();

    stdin.lock()
        .read_until(0, &mut buffer).unwrap();

    let data: String = String::from_utf8(buffer).unwrap();

    //println!("A: {}", solution_a(&data));
    //println!("B: {}", solution_b(&data));
}
