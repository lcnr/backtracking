//! a solver of the n queens problem using backtracking

use crate::Sequence;

use primal_bit::BitVec;

use std::ops::Range;

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
    type Steps = Range<Self::Step>;

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

    fn next_steps(&self) -> Self::Steps {
        0..self.n
    }

    fn apply_step(&self, step: Self::Step) -> Self {
        let mut rows = self.rows.clone();
        rows.push(step);
        Self { n: self.n, rows }
    }
}

/// Solves the n queens problem using the b* algorithm described on page 4 of pre-fascicle 5B.
///
/// This solution is far more efficient than the algorithm b, but can only be used for this exact problem.
pub fn b_star(n: usize) -> Vec<Queens> {
    let mut results = Vec::new();
    // currently occupied rows
    let mut a = BitVec::from_elem(n, false);
    // diagonal lines going downwards.
    // accessed with `row + column`
    let mut b = BitVec::from_elem(2 * n - 1, false);

    // diagonal lines going to the upwards
    // accessed with `row + (n - 1) - column`
    let mut c = BitVec::from_elem(2 * n - 1, false);

    let mut positions = Vec::new();

    let mut t = 0;
    loop {
        while t < n {
            if !(a[t] || b[t + positions.len()] || c[t + n - 1 - positions.len()]) {
                if positions.len() + 1 < n {
                    a.set(t, true);
                    b.set(t + positions.len(), true);
                    c.set(t + n - 1 - positions.len(), true);
                    positions.push(t);
                    t = 0;
                } else {
                    let mut q = positions.clone();
                    q.push(t);
                    results.push(Queens { n, rows: q });
                    t += 1;
                }
            } else {
                t += 1;
            }
        }

        if let Some(prev) = positions.pop() {
            c.set(prev + n - 1 - positions.len(), false);
            b.set(prev + positions.len(), false);
            a.set(prev, false);
            t = prev + 1;
        } else {
            break;
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b;

    #[test]
    fn four() {
        let b = b(Queens::new(4), 4);
        assert_eq!(b.len(), 2);

        assert!(b.iter().any(|q| q.rows == &[1, 3, 0, 2]));
        assert!(b.iter().any(|q| q.rows == &[2, 0, 3, 1]));

        let b_star = b_star(4);
        assert_eq!(b_star.len(), 2);

        assert!(b.iter().any(|q| q.rows == &[1, 3, 0, 2]));
        assert!(b.iter().any(|q| q.rows == &[2, 0, 3, 1]));
    }

    #[test]
    fn eight() {
        assert_eq!(b(Queens::new(8), 8).len(), 92);
        assert_eq!(b_star(8).len(), 92);
    }
}
