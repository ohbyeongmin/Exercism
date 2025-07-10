pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index = (student.as_bytes().first().cloned().unwrap() - b'A') as usize * 2;
    diagram
        .chars()
        .enumerate()
        .filter_map(|(i, p)| {
            if i == index
                || i == index + 1
                || i == index + diagram.len() / 2 + 1
                || i == index + diagram.len() / 2 + 2
            {
                match p {
                    'G' => Some("grass"),
                    'C' => Some("clover"),
                    'R' => Some("radishes"),
                    'V' => Some("violets"),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect()
}
