pub fn reply(message: &str) -> &str {
    dbg!(
        message,
        is_question(message),
        is_yelled(message),
        is_saying_nothing(message)
    );
    if is_yelled_question(message) {
        "Calm down, I know what I'm doing!"
    } else if is_saying_nothing(message) {
        "Fine. Be that way!"
    } else if is_question(message) {
        "Sure."
    } else if is_yelled(message) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}

fn is_question(message: &str) -> bool {
    message.trim_end().chars().rev().position(|x| x == '?') == Some(0)
}

fn is_yelled(message: &str) -> bool {
    let tmp: Vec<char> = message
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .collect();

    tmp.len() > 0 && tmp.iter().all(|x| x.is_ascii_uppercase())
}

fn is_yelled_question(message: &str) -> bool {
    is_question(message) && is_yelled(message)
}

fn is_saying_nothing(message: &str) -> bool {
    message.trim().is_empty()
}
