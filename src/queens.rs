//! a solver of the n queens problem using backtracking

use crate::b::Sequence;

pub struct Queens {
    n: usize,
    rows: Vec<usize>,
}

impl Queens {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            rows: Vec::new(),
        }
    }
}

impl Sequence for Queens {
    type Step = usize;

    fn satisfies_condition(&self) -> bool {
        for k in 0..self.rows.len() {
            for j in 0..k {
                let k_col = self.rows[k];
                let j_col = self.rows[j];
                // this is identical to `(k_col - j_col).abs() != k - j` without requiring signed integers
                if k_col == j_col || k_col.max(j_col) - k_col.min(j_col) == k - j {
                    return false;
                }
            }
        }

        true
    }

    fn next_steps(&self) -> Vec<Self::Step> {
        (0..self.n).collect()
    }

    fn apply_step(&self, step: Self::Step) -> Self {
        let mut rows = self.rows.clone();
        rows.push(step);
        Self { n: self.n, rows }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b::b;

    #[test]
    fn four() {
        let b = b(Queens::new(4), 4);
        assert_eq!(b.len(), 2);

        assert!(b.iter().any(|q| q.rows == &[1, 3, 0, 2]));
        assert!(b.iter().any(|q| q.rows == &[2, 0, 3, 1]));
    }

    #[test]
    fn eight() {
        assert_eq!(b(Queens::new(8), 8).len(), 92);   
    }
}
