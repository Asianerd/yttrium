use std::io;

pub fn read_line() -> String {
    let mut input = "".to_string();
    io::stdin().read_line(&mut input).unwrap();
    input.strip_suffix("\n").unwrap().to_string()
}