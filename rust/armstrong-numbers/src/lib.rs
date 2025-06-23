pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let power = num_string.len() as u32;

    num_string
        .bytes()
        .fold(0_u32, |acc, x| u32::pow((x - b'0').into(), power) + acc)
        == num
}
