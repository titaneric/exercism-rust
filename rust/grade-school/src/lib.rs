// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
use std::collections::{BTreeMap, BTreeSet};
#[allow(clippy::new_without_default)]
pub struct School<'a> {
    grade: BTreeMap<u32, BTreeSet<&'a str>>,
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        School {
            grade: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        let entry = self.grade.entry(grade).or_insert(BTreeSet::new());
        entry.insert(student);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grade.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grade
            .get(&grade)
            .unwrap_or(&BTreeSet::new())
            .iter()
            .map(|&s| String::from(s))
            .collect()
    }
}
