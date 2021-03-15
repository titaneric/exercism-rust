pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stk = vec![];
    for c in string.chars() {
        match c {
            '[' | '{' | '(' => stk.push(c),
            ']' | '}' | ')' => {
                if let Some(top) = stk.pop() {
                    if !is_matched_brackets(top, c) {
                        return false;
                    }
                } else {
                    return false
                }
            }
            _ => continue,
        }
    }
    if stk.is_empty() {
        true
    } else {
        false
    }
}
fn is_matched_brackets(s: char, t: char) -> bool {
    let refs = [('[', ']'), ('(', ')'), ('{', '}')];
    for (l, r) in &refs {
        if l == &s && r == &t {
            return true;
        }
    }
    false
}
