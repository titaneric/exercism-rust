/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false;
    }
    let valid_chars: Vec<_> = isbn
        .chars()
        .filter(|&c| c.is_ascii_digit() || c == 'X')
        .collect();

    let is_last_x = valid_chars.iter().position(|&c| c == 'X').or(Some(9)) == Some(9);
    if valid_chars.len() == 10 && is_last_x {
        valid_chars
            .iter()
            .zip((1..=10).rev())
            .fold(0, |acc, (num, idx)| {
                let num = match num {
                    'X' => 10,
                    _ => num.to_digit(10).unwrap(),
                };
                acc + (num * idx)
            })
            .rem_euclid(11)
            == 0
    } else {
        false
    }
}
