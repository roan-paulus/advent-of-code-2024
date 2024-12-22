use std::fs;

fn main() {
    let challenge_input = fs::read_to_string("input.txt").unwrap();

    let sum = challenge_input
        .split('\n')
        .filter(|report: &&str| is_safe(report))
        .count();

    println!("We have {} safe rows", sum);
}

fn is_safe(report: &str) -> bool {
    let numbers: Vec<i32> = report
        .split(' ')
        .take_while(|number| !number.is_empty())
        .map(|number| {
            number.parse().expect(
                &format!("expected number to be a number but it is: '{number}'").to_string(),
            )
        })
        .collect();

    const MIN_CHANGE: i32 = 1;
    const MAX_CHANGE: i32 = 3;

    if numbers.len() < 2 {
        return false;
    }

    let is_decreasing = numbers[0] > numbers[1];

    for i in 0..numbers.len() - 1 {
        let curr = numbers[i];
        let next = numbers[i + 1];
        if is_decreasing {
            if next > curr {
                return false;
            }
        } else if next < curr {
            return false;
        }
        let mut diff = next - curr;
        if diff < 0 {
            diff *= -1;
        }
        if diff < MIN_CHANGE || diff > MAX_CHANGE {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_from_page() {
        assert_eq!(is_safe("7 6 4 2 1"), true);
        assert_eq!(is_safe("1 2 7 8 9"), false);
        assert_eq!(is_safe("9 7 6 2 1"), false);
        assert_eq!(is_safe("1 3 2 4 5"), false);
        assert_eq!(is_safe("8 6 4 4 1"), false);
        assert_eq!(is_safe("1 3 6 7 9"), true);
    }
}
