use std::collections::BTreeMap;
pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(score, alphas)| {
            alphas
                .into_iter()
                .zip([score].iter().cycle())
                .map(|(&alpha, &&score)| (alpha.to_ascii_lowercase(), score))
                .collect::<Vec<(char, i32)>>()
        })
        .collect()
}
