fn is_prime(num: u32) -> bool {
    let mut index = 2;
    while index * index <= num {
        if num % index == 0 {
            return false;
        }
        index += 1;
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut num = 2;
    while count <= n {
        if is_prime(num) {
            count += 1;
        }
        num += 1;
    }
    num - 1
}
