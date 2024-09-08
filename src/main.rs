use std::env;

fn char_to_keypad(c: char) -> Option<char> {
    match c {
        'A' | 'B' | 'C' => Some('2'),
        'D' | 'E' | 'F' => Some('3'),
        'G' | 'H' | 'I' => Some('4'),
        'J' | 'K' | 'L' => Some('5'),
        'M' | 'N' | 'O' => Some('6'),
        'P' | 'Q' | 'R' | 'S' => Some('7'),
        'T' | 'U' | 'V' => Some('8'),
        'W' | 'X' | 'Y' | 'Z' => Some('9'),
        '1'..='9' | '0' => Some(c),
        ' ' => Some('0'),
        _ => None,
    }
}

fn convert_to_keypad(input: &str) -> Result<String, String> {
    let mut result = String::new();
    for c in input.to_uppercase().chars() {
        match char_to_keypad(c) {
            Some(num) => result.push(num),
            None => return Err(format!("Error: Invalid character '{}'", c)),
        }
    }
    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: <program> <string>");
        return;
    }

    match convert_to_keypad(&args[1]) {
        Ok(keypad_code) => println!("Keypad code: {}", keypad_code),
        Err(err) => println!("{}", err),
    }
}

