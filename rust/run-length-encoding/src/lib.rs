// copy from https://exercism.io/tracks/rust/exercises/run-length-encoding/solutions/ef3b5fed37924b2fb0db6db051e2a14c
pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let first_char = source.chars().next().unwrap();
    let mut num_char = vec![(first_char, 1)];

    for c in source.chars().skip(1) {
        if c == num_char.last().unwrap().0 {
            num_char.last_mut().unwrap().1 += 1;
        } else {
            num_char.push((c, 1));
        }
    }
    num_char
        .into_iter()
        .map(|(c, n)| {
            if n == 1 {
                format!("{}", c)
            } else {
                format!("{}{}", n, c)
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut cnt = 0;

    for s in source.chars() {
        if s.is_ascii_digit() {
            cnt = cnt * 10 + s.to_digit(10).unwrap();
        } else {
            let mut repeat = cnt as usize;
            if repeat == 0 {
                repeat = 1;
            }
            result.push_str(&format!("{}", s).repeat(repeat));
            cnt = 0;
        }
    }

    result
}
