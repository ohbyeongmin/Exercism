pub fn brackets_are_balanced(string: &str) -> bool {
    let brackets = string
        .as_bytes()
        .iter()
        .filter(|&&item| matches!(item, b'(' | b'{' | b'[' | b')' | b'}' | b']'))
        .copied()
        .collect::<Vec<_>>();

    if brackets.is_empty() {
        return true;
    }

    let mut stack: Vec<u8> = vec![];

    for item in brackets {
        match item {
            b'(' | b'{' | b'[' => stack.push(item),
            b')' => match stack.pop() {
                Some(item) => {
                    if item != b'(' {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            },
            b'}' => match stack.pop() {
                Some(item) => {
                    if item != b'{' {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            },
            b']' => match stack.pop() {
                Some(item) => {
                    if item != b'[' {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            },
            _ => (),
        }
    }
    stack.is_empty()
}
