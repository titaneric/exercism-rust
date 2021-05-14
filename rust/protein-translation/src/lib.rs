use std::collections::HashMap;
pub struct CodonsInfo<'a> {
    mapping: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.mapping.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(|codon| self.name_for(std::str::from_utf8(codon).unwrap()))
            .take_while(|&name| name != Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        mapping: pairs.into_iter().collect(),
    }
}
