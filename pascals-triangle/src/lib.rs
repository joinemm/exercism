pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle = Self { rows: vec![] };
        for r in 0..row_count {
            triangle.rows.push(vec![]);
            for c in 0..=r {
                triangle.rows[r as usize].push(PascalsTriangle::calculate(r, c));
            }
        }
        triangle
    }

    fn calculate(row: u32, term: u32) -> u32 {
        factorial(row) / (factorial(term) * factorial(row - term))
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

fn factorial(num: u32) -> u32 {
    match num {
        0 | 1 => 1,
        _ => factorial(num - 1) * num,
    }
}
