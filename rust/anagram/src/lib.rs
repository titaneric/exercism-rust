use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_map = build_dict(word);
    possible_anagrams
        .iter()
        .map(|&candidate| (candidate, build_dict(candidate)))
        .filter_map(|(candidate, candidate_map)| {
            if (candidate_map == word_map) && (candidate.to_lowercase() != word.to_lowercase()) {
                Some(candidate)
            } else {
                None
            }
        })
        .collect()
}

fn build_dict(word: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    word.to_lowercase().chars().for_each(|c| {
        let counter = map.entry(c).or_insert(0u32);
        *counter += 1;
    });
    map
}
