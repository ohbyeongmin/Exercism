pub fn egg_count(display_value: u32) -> usize {
    let mut value = display_value;
    let mut count: usize = 0;

    loop {
        if value == 0 {
            break;
        }
        if value % 2 != 0 {
            count += 1;
        }
        value >>= 1;
    }
    count
}
