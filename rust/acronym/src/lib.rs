pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .filter(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace())
        .map(|c| if c == '-' { ' ' } else { c })
        .collect::<String>()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|word| {dbg!(word);word.chars().next().unwrap().to_uppercase().to_string()})
        .collect()
}
