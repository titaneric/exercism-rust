use std::collections::BTreeSet;
use std::ops::Add;
use std::default::Default;
pub struct Triangle<T> {
    unique_edge: BTreeSet<T>,
}

impl<T> Triangle<T> 
where T: Add<Output = T> + Ord + Clone + Default + Copy
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let mut sides = sides;
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if sides.iter().all(|&s| s > Default::default()) && ((sides[0] + sides[1]) > sides[2]) {
            return Some(Triangle {
                unique_edge: sides.iter().cloned().collect(),
            });
        }
        None
    }

    pub fn is_equilateral(&self) -> bool {
        self.unique_edge.len() == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.unique_edge.len() == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.unique_edge.len() < 3
    }
}
