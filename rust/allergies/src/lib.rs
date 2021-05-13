pub struct Allergies(u32);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}
const NUM_ALLERGY: usize = 7;
impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        ((self.0 >> (*allergen as u32)) & 1) == 1
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;

        format!("{:b}", self.0)
            .chars()
            .rev()
            .zip(0..)
            .filter_map(|(chr, idx)| {
                let allergy_type = match (idx, chr) {
                    (_, _) if idx > NUM_ALLERGY => None,
                    (_, _) if chr == '0' => None,
                    (0, _) => Some(Eggs),
                    (1, _) => Some(Peanuts),
                    (2, _) => Some(Shellfish),
                    (3, _) => Some(Strawberries),
                    (4, _) => Some(Tomatoes),
                    (5, _) => Some(Chocolate),
                    (6, _) => Some(Pollen),
                    (7, _) => Some(Cats),
                    (_, _) => None,
                };
                allergy_type
            })
            .collect()
    }
}
