pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split_whitespace()
        .flat_map(|a| a.split('-').collect::<Vec<_>>())
        .map(|a| {
            a.chars()
                .filter(|&c| c.is_ascii_alphabetic())
                .collect::<String>()
        })
        .filter(|s| !s.is_empty())
        .map(|a| {
            if a.chars().all(|c| c.is_uppercase()) {
                a.chars().next().unwrap().to_string()
            } else {
                let mut chars = a.chars();
                let mut result = String::new();

                result.push(chars.next().unwrap());

                for c in chars {
                    if c.is_uppercase() {
                        result.push(c);
                    }
                }

                result
            }
        })
        .collect::<String>()
        .to_uppercase()
}
