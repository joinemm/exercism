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
        match self.scores.last() {
            Some(value) => Some(*value),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.iter().max() {
            Some(value) => Some(*value),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut temp = self.scores.to_vec();
        temp.sort_by(|a, b| a.cmp(b).reverse());
        temp.truncate(3);
        temp
    }
}
