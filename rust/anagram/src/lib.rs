use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let target_word = word.to_lowercase();
    let filtered_same_words = possible_anagrams
        .iter()
        .filter(|word| target_word != word.to_lowercase())
        .copied()
        .collect::<Vec<_>>();

    let mut vec_target = target_word.chars().collect::<Vec<_>>();
    vec_target.sort_unstable();

    let sorted_target = vec_target.into_iter().collect::<String>();

    filtered_same_words
        .iter()
        .filter(|word| {
            let lower_word = word.to_lowercase();
            let mut vec_word = lower_word.chars().collect::<Vec<_>>();
            vec_word.sort_unstable();

            let sorted_word = vec_word.into_iter().collect::<String>();

            sorted_target == sorted_word
        })
        .copied()
        .collect::<HashSet<&'a str>>()
}
