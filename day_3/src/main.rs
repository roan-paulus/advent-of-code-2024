use std::{
    fs,
    iter::{Enumerate, Peekable},
    str::Chars,
};

type PEChars<'a> = Peekable<Enumerate<Chars<'a>>>;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    assert!(input.is_ascii());

    let result = calculate(input);
    println!("Sum of all parsed expression = {result}");
}

fn calculate(input: String) -> i32 {
    let mut mul_enabled = true;
    let mut multiplied_numbers: Vec<i32> = Vec::new();

    let mut chars: PEChars = input.chars().enumerate().peekable();
    while let Some((i, ch)) = chars.next() {
        match ch {
            'm' if mul_enabled => {
                let end = i + 4;
                assert!(end < input.len());
                let slice = &input[i..end];

                if slice != "mul(" {
                    continue;
                }
                // Consume "mul("
                for _ in 1..=3 {
                    chars.next();
                }

                match chars.peek() {
                    Some(ch) if ch.1.is_digit(10) => (),
                    _ => continue,
                }

                let Some((a, b)) = read_numbers(&mut chars) else {
                    continue;
                };
                multiplied_numbers.push(a * b);
            }
            'd' => {
                let slice_size = 4;
                let end = i + slice_size;
                assert!(end < input.len());
                let slice = &input[i..end];

                if slice == "do()" {
                    mul_enabled = true;
                    // Consume "do()"
                    for _ in 1..slice_size {
                        chars.next();
                    }
                    continue;
                }

                let slice_size = 7;
                let end = i + slice_size;
                assert!(end < input.len());
                let slice = &input[i..end];

                if slice == "don't()" {
                    mul_enabled = false;
                    // Consume "don't()"
                    for _ in 1..slice_size {
                        chars.next();
                    }
                    continue;
                }
            }
            _ => (),
        }
    }
    multiplied_numbers.into_iter().sum()
}

fn read_numbers(chars: &mut PEChars) -> Option<(i32, i32)> {
    let mut a = Vec::new();
    while let Some((_, ch)) = chars.peek() {
        if *ch == ',' {
            chars.next();
            break;
        } else if !ch.is_digit(10) {
            return None;
        }
        a.push(*ch);
        chars.next();
    }

    // Copied top loop but for b.
    let mut b = Vec::new();
    while let Some((_, ch)) = chars.peek() {
        if *ch == ')' {
            chars.next();
            break;
        } else if !ch.is_digit(10) {
            return None;
        }
        b.push(*ch);
        chars.next();
    }

    Some((convert_to_number(a), convert_to_number(b)))
}

fn convert_to_number(chars: Vec<char>) -> i32 {
    let mut number = 0;
    let mut place = chars.len() as u32;
    for char in chars {
        place -= 1;
        // Already known to be a digit so unwrap it.
        let digit = char.to_digit(10).unwrap() as i32;
        number += digit * 10_i32.pow(place);
    }
    number
}
