fn make_bottles(count_bottles: u32) -> String {
    if count_bottles == 10 {
        "Ten green bottles".to_string()
    } else if count_bottles == 9 {
        "Nine green bottles".to_string()
    } else if count_bottles == 8 {
        "Eight green bottles".to_string()
    } else if count_bottles == 7 {
        "Seven green bottles".to_string()
    } else if count_bottles == 6 {
        "Six green bottles".to_string()
    } else if count_bottles == 5 {
        "Five green bottles".to_string()
    } else if count_bottles == 4 {
        "Four green bottles".to_string()
    } else if count_bottles == 3 {
        "Three green bottles".to_string()
    } else if count_bottles == 2 {
        "Two green bottles".to_string()
    } else if count_bottles == 1 {
        "One green bottle".to_string()
    } else {
        "No green bottles".to_string()
    }
}

fn create_verse(count_bottles: u32) -> String {
    "\n".to_string()
        + &hanging_part(count_bottles).repeat(2)
        + "And if one green bottle should accidentally fall,\n"
        + &after_accident_part(count_bottles - 1)
}

fn hanging_part(count_bottles: u32) -> String {
    format!("{} hanging on the wall,\n", make_bottles(count_bottles))
}

fn after_accident_part(count_bottles: u32) -> String {
    format!(
        "There'll be {} hanging on the wall.\n",
        make_bottles(count_bottles).to_lowercase()
    )
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (start_bottles - take_down + 1..=start_bottles)
        .rev()
        .fold("".to_string(), |acc, x| acc + &create_verse(x))
}
