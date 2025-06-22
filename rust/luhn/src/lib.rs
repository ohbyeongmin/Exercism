/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let bytes_of_code = code
        .split(' ')
        .flat_map(|item| item.as_bytes())
        .collect::<Vec<_>>();

    if bytes_of_code.len() <= 1 {
        return false;
    }

    if !bytes_of_code
        .iter()
        .all(|&item| *item >= b'0' && *item <= b'9')
    {
        return false;
    }

    bytes_of_code
        .iter()
        .rev()
        .map(|&value| (*value - b'0') as u32)
        .enumerate()
        .map(|(index, value)| {
            if index % 2 != 0 {
                let convert = value * 2;
                if convert > 9 {
                    convert - 9
                } else {
                    convert
                }
            } else {
                value
            }
        })
        .reduce(|acc, e| acc + e)
        .unwrap()
        % 10
        == 0
}
