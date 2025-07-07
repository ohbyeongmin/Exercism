static AROUND_OFFSETS: &[(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len() as i32;
    (0..height)
        .map(|y| {
            let width = garden[y as usize].len() as i32;
            (0..width)
                .map(|x| {
                    if garden[y as usize].as_bytes()[x as usize] == b'*' {
                        '*'
                    } else {
                        match AROUND_OFFSETS
                            .iter()
                            .map(|&(ox, oy)| (x + ox, y + oy))
                            .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
                            .filter(|&(x, y)| garden[y as usize].as_bytes()[x as usize] == b'*')
                            .count()
                        {
                            0 => ' ',
                            n => (n as u8 + b'0') as char,
                        }
                    }
                })
                .collect()
        })
        .collect()
}
