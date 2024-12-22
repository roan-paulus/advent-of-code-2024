use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("input.txt").expect("There should be a file there");

    const SIZE: usize = 16384;
    let mut buf: [u8; SIZE] = [0; SIZE];
    let bytes_read = f.read(&mut buf).unwrap();
    println!("Read {bytes_read} bytes");

    let (rules, updates) = split_rules_updates(&buf);
}

fn split_rules_updates(input: &[u8]) -> (Vec<char>, Vec<char>) {
    let middle = input
        .windows(2)
        .position(|window| window == [b'\n', b'\n'])
        .expect("There should be two newlines seperating the rules from the updates");

    let rules = to_vec_chars(&input[..middle]);
    let updates = to_vec_chars(&input[middle..]);

    (rules, updates)
}

fn to_vec_chars(slice: &[u8]) -> Vec<char> {
    slice.iter().map(|byte| *byte as char).collect()
}
