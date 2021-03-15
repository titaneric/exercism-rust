pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = vec![];
        for r in 0..row_count {
            let mut row = vec![];
            for i in 0..=r {
                row.push(comb_num(r, i));
            }
            rows.push(row);
        }
        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.to_vec()
    }
}
fn comb_num(n: u32, k: u32) -> u32 {
    factorial(n) / (factorial(k) * factorial(n - k))
}
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
