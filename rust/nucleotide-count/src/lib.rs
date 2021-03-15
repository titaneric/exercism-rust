use std::collections::HashMap;
const NUCLEOTIDE: &str = "ATCG";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDE.contains(nucleotide) {
        return Err('X');
    }
    dna.chars().try_fold(0, |cnt, c| {
        if !NUCLEOTIDE.contains(c) {
            Err(c)
        } else {
            Ok(cnt + (c == nucleotide) as usize)
        }
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    NUCLEOTIDE
        .chars()
        .map(|chr| Ok((chr, count(chr, dna)?)))
        .collect()
}
