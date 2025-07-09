pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .as_bytes()
        .windows(len)
        .map(|strings| {
            strings.iter().fold("".to_string(), |acc, &x| {
                let mut temp = acc;
                temp.push(x as char);
                temp
            })
        })
        .collect()
}
