fn is_prime(num: u64) -> bool {
    let mut index = 2;
    while index * index <= num {
        if num % index == 0 {
            return false;
        }
        index += 1;
    }
    true
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut divisor = 2u64;
    let mut results: Vec<u64> = vec![];
    while num != 1 {
        if !is_prime(divisor) {
            divisor += 1;
            continue;
        }

        if num % divisor != 0 {
            divisor += 1;
            continue;
        }

        num /= divisor;
        results.push(divisor);
    }

    results
}
