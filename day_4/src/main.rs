use std::{fs::File, io::Read, str::SplitWhitespace};

fn main() {
    let mut input = String::new();
    let mut f = match File::open("input.txt") {
        Ok(f) => f,
        Err(_) => panic!("File not found"),
    };
    f.read_to_string(&mut input).expect("Should be valid UTF-8");

    let count = count_xmas_occurences(input);
    println!("XMAS was found {count} times");
}

type TwoDimensional = Vec<Vec<char>>;

fn count_xmas_occurences(text: String) -> u32 {
    let lines = text.split_whitespace();

    // let horizontal_occurences = XMASScanner::horizontal(&lines);

    let lines: TwoDimensional = lines.map(|line| line.chars().collect()).collect();
    XMASScanner::cross(&lines)

    // let vertical_occurences = XMASScanner::vertical(&lines);
    // let diagonal_occurences = XMASScanner::diagonal(&lines);

    // horizontal_occurences + vertical_occurences + diagonal_occurences
}

struct XMASScanner;

const WORD_SIZE: usize = 4;

fn is_xmas(s: &str) -> bool {
    s == "XMAS" || s == "SAMX"
}

impl XMASScanner {
    fn horizontal(lines: &SplitWhitespace) -> u32 {
        let mut count = 0;
        lines.clone().for_each(|line| {
            line.as_bytes().windows(WORD_SIZE).for_each(|w| {
                let w: String = w.iter().map(|ch| *ch as char).collect();
                if is_xmas(&w) {
                    count += 1;
                }
            });
        });
        count
    }
    fn vertical(lines: &TwoDimensional) -> u32 {
        let mut count = 0;
        let line_length = lines[0].len();
        for x in 0..line_length {
            for y in 0..lines.len() {
                assert!(lines[y].len() == line_length);

                let offset = WORD_SIZE;
                let end = y + offset;

                let overflow = end > lines.len();
                if overflow {
                    break;
                }

                let mut window = String::new();
                (y..end).for_each(|y| {
                    window.push(lines[y][x]);
                });

                if is_xmas(&window) {
                    count += 1;
                }
            }
        }
        count
    }
    fn diagonal(lines: &TwoDimensional) -> u32 {
        let mut count = 0;

        for y in 0..lines.len() {
            let vertical_overflow = y + WORD_SIZE > lines.len();
            if vertical_overflow {
                break;
            }
            for x in 0..lines[y].len() {
                let horizontal_overflow = x + WORD_SIZE > lines[y].len();
                if horizontal_overflow {
                    break;
                }
                let mut window_backward_slash = String::new();
                let mut window_forward_slash = String::new();
                for i in 0..WORD_SIZE {
                    window_backward_slash.push(lines[y + i][x + i]);
                    let x = x + WORD_SIZE - 1;
                    window_forward_slash.push(lines[y + i][x - i]);
                }

                if is_xmas(&window_backward_slash) {
                    count += 1;
                }
                if is_xmas(&window_forward_slash) {
                    count += 1;
                }
            }
        }
        count
    }

    fn cross(lines: &TwoDimensional) -> u32 {
        let mut count = 0;

        for y in 0..lines.len() {
            let vertical_overflow = y + CROSS_SIZE > lines.len();
            if vertical_overflow {
                break;
            }
            for x in 0..lines[y].len() {
                let horizontal_overflow = x + CROSS_SIZE > lines[y].len();
                if horizontal_overflow {
                    break;
                }
                let mut window_backward_slash = String::new();
                let mut window_forward_slash = String::new();
                for i in 0..CROSS_SIZE {
                    window_backward_slash.push(lines[y + i][x + i]);
                    let x = x + CROSS_SIZE - 1;
                    window_forward_slash.push(lines[y + i][x - i]);
                }

                if is_crossmas(&window_backward_slash) && is_crossmas(&window_forward_slash) {
                    count += 1;
                }
            }
        }
        count
    }
}

const CROSS_SIZE: usize = 3;

fn is_crossmas(s: &str) -> bool {
    s == "MAS" || s == "SAM"
}

#[cfg(test)]
mod tests {
    fn vert_dia_input(s: &str) -> TwoDimensional {
        let lines: TwoDimensional = s.split('\n').map(|line| line.chars().collect()).collect();
        lines
    }

    use super::*;
    #[test]
    fn vertical_works() {
        let lines = vert_dia_input("X....\nM....\nA....\nS....");
        let result = XMASScanner::vertical(&lines);
        assert_eq!(result, 1);
    }

    #[test]
    fn no_vertical_christmas() {
        let lines = vert_dia_input(".....\n.....\n.....\n.....");
        let result = XMASScanner::vertical(&lines);
        assert_eq!(result, 0);
    }

    #[test]
    fn diagonal_from_top_left_xmas_found() {
        let lines = vert_dia_input("X....\n.M...\n..A..\n...S.");
        let result = XMASScanner::diagonal(&lines);
        assert_eq!(result, 1);
    }

    #[test]
    fn diagonal_from_bottom_left_xmas_found() {
        let lines = vert_dia_input("...S.\n..A..\n.M...\nX....");
        let result = XMASScanner::diagonal(&lines);
        assert_eq!(result, 1);
    }

    #[test]
    fn eighteen_times_present_in_text() {
        let input = "MMMSXXMASM
                    MSAMXMSMSA
                    AMXSXMAAMM
                    MSAMASMSMX
                    XMASAMXAMM
                    XXAMMXXAMA
                    SMSMSASXSS
                    SAXAMASAAA
                    MAMMMXMMMM
                    MXMXAXMASX\n\n\n\n"
            .to_string();
        let count = count_xmas_occurences(input);
        assert_eq!(count, 18);
    }

    #[test]
    fn three_times_in_text() {
        let input = "XMAS...
                     MM.....
                     A.A....
                     S..S..."
            .to_string();
        let count = count_xmas_occurences(input);
        assert_eq!(count, 3);
    }

    #[test]
    fn three_times_in_text_2() {
        let input = "SAMX...
                     AA.....
                     M.M....
                     X..X..."
            .to_string();
        let count = count_xmas_occurences(input);
        assert_eq!(count, 3);
    }

    #[test]
    fn five_time_in_test() {
        let input = "SAMX.XS
                     AA..MA.
                     M.MAM..
                     X.SX..."
            .to_string();
        let count = count_xmas_occurences(input);
        assert_eq!(count, 5);
    }

    #[test]
    fn overflow() {
        let input = ".XXX...
                     MMM....
                     AA.....
                     S......"
            .to_string();
        let count = count_xmas_occurences(input);
        assert_eq!(count, 1);
    }

    #[test]
    fn two_diagonally_at_the_same_time() {
        let input = "X..S...
                     .MA....
                     .MA....
                     X..S..."
            .to_string();
        let count = count_xmas_occurences(input);
        assert_eq!(count, 2);
    }
}
