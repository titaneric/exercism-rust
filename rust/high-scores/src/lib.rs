#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.len() > 1 {
            Some(self.scores[self.scores.len() - 1])
        } else {
            None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut sorted_scores = self.scores.iter().collect::<Vec<_>>();
        sorted_scores.sort();
        if self.scores.len() > 1 {
            Some(*sorted_scores[sorted_scores.len() - 1])
        } else {
            None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_scores = self.scores.iter().collect::<Vec<_>>();
        sorted_scores.sort();
        sorted_scores.iter().rev().take(3).map(|&&x| x).collect()
    }
}
