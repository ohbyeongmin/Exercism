enum Reply {
    Sure,
    WhoaChillOut,
    CalmDown,
    Fine,
    Whatever,
}

fn is_all_upper(message: &str) -> bool {
    let a = message
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>();

    !a.is_empty() && a.chars().all(|c| c.is_ascii_uppercase())
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}

fn is_all_whitespace(message: &str) -> bool {
    message.chars().all(|c| c.is_ascii_whitespace())
}

impl Reply {
    fn new(message: &str) -> Self {
        if is_all_upper(message) && is_question(message) {
            Reply::CalmDown
        } else if is_all_upper(message) {
            Reply::WhoaChillOut
        } else if is_question(message) {
            Reply::Sure
        } else if is_all_whitespace(message) {
            Reply::Fine
        } else {
            Reply::Whatever
        }
    }
}

pub fn reply(message: &str) -> &str {
    match Reply::new(message.trim()) {
        Reply::Sure => "Sure.",
        Reply::WhoaChillOut => "Whoa, chill out!",
        Reply::CalmDown => "Calm down, I know what I'm doing!",
        Reply::Fine => "Fine. Be that way!",
        Reply::Whatever => "Whatever.",
    }
}
