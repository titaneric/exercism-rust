#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

const DNA_SEQ: &'static str = "ACGT";
const RNA_SEQ: &'static str = "ACGU";

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.chars()
            .position(|n| !DNA_SEQ.contains(n))
            .map_or(Ok(Dna(dna.to_string())), Err)
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'G' => 'C',
                'T' => 'A',
                'C' => 'G',
                _ => unreachable!(),
            })
            .collect::<String>())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.chars()
            .position(|n| !RNA_SEQ.contains(n))
            .map_or(Ok(Rna(rna.to_string())), Err)
    }
}
