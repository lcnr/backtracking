//! solvers for the n queen problem

use crate::Sequence;

use primal_bit::BitVec;

use std::ops::Range;

#[derive(Clone)]
pub struct Queens {
    /// dimension of the chessboard
    n: usize,
    /// currently occupied rows
    rows: Vec<usize>,
}

impl Queens {
    /// creates a new empty chess board of size `n`
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

    /// checks if the most recently placed queen was placed on a free tile
    fn satisfies_condition(&self) -> bool {
        if self.rows.is_empty() {
            return true;
        }

        // the row of the last queen
        let k = self.rows.len() - 1;

        // for all previous queens
        for j in 0..self.rows.len() - 1 {
            let k_col = self.rows[k] as isize;
            let j_col = self.rows[j] as isize;

            // check if the queen at row `j` shares a column or diagonal
            // with the queen at row `k`
            if k_col == j_col || (j_col - k_col).abs() as usize == k - j {
                return false;
            }
        }
        true
    }

    /// returns all possible columns
    fn next_steps(&self) -> Self::Steps {
        0..self.n
    }

    /// clones `self` and adds the queen at `column` at the next free row
    fn apply_step(&self, column: Self::Step) -> Self {
        let mut rows = self.rows.clone();
        rows.push(column);
        Self { n: self.n, rows }
    }
}

/// Solves the n queens problem using the b* algorithm described on page 4 of pre-fascicle 5B.
///
/// This solution is far more efficient than the algorithm b, but can only be used for this exact problem.
pub fn b_star(n: usize) -> Vec<Queens> {
    let mut results = Vec::new();

    // attacked columns, should be accessed with `column`
    let mut columns = BitVec::from_elem(n, false);

    // attacked diagonal lines going to the left,
    // should be accessed with `row + column`
    //
    // For `n == 4` this can be visualized as
    //
    // ```plain
    // 0123
    // 1234
    // 2345
    // 3456
    // ```
    let mut left_diagonals = BitVec::from_elem(2 * n - 1, false);

    // attacked diagonal lines going to the right,
    // should be accessed with `column + (n - 1) + row`
    //
    // For `n == 4` this can be visualized as
    //
    // ```plain
    // 3456
    // 2345
    // 1234
    // 0123
    // ```
    let mut right_diagonals = BitVec::from_elem(2 * n - 1, false);

    // all currently occupied rows
    let mut rows = Vec::new();
    // the currently tried column
    let mut column = 0;

    loop {
        // test all possible columns
        while column < n {
            // check if the current position is on an
            // already occupied column or diagonal
            if !(columns[column]
                || left_diagonals[column + rows.len()]
                || right_diagonals[column + n - 1 - rows.len()])
            {
                if rows.len() + 1 < n {
                    columns.set(column, true);
                    left_diagonals.set(column + rows.len(), true);
                    right_diagonals.set(column + n - 1 - rows.len(), true);
                    rows.push(column);
                    column = 0;
                } else {
                    // add the new state to results,
                    // do not bother to update attacked columns and diagonals,
                    // as these changes would have to be instantly reverted anyways
                    let mut q = rows.clone();
                    q.push(column);
                    results.push(Queens { n, rows: q });
                    column += 1;
                }
            } else {
                column += 1;
            }
        }

        // revert the last step, updating `column`
        if let Some(prev) = rows.pop() {
            right_diagonals.set(prev + n - 1 - rows.len(), false);
            left_diagonals.set(prev + rows.len(), false);
            columns.set(prev, false);
            column = prev + 1;
        } else {
            return results;
        }
    }
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
