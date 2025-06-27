pub fn is_leap_year(year: u64) -> bool {
    match year % 4 == 0 {
        true => match year % 100 == 0 {
            true => match year % 400 == 0 {
                true => true,
                false => false,
            },
            false => true,
        },
        false => false,
    }
}
