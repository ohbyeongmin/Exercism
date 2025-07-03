use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter_map(|&f| {
            if f != 0 {
                Some(
                    (1..=(limit / f))
                        .filter_map(|n| if n * f < limit { Some(n * f) } else { None })
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect::<HashSet<_>>()
        .into_iter()
        .sum()
}
