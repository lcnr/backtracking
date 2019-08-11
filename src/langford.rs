use crate::Sequence;

#[derive(Clone)]
pub struct LangfordPairsBrute {
    n: isize,
    values: Vec<isize>,
}

impl LangfordPairsBrute {
    pub fn new(n: isize) -> Self {
        Self {
            n,
            values: Vec::new(),
        }
    }
}

impl Sequence for LangfordPairsBrute {
    type Step = isize;
    type Steps = Vec<isize>;

    fn satisfies_condition(&self) -> bool {
        // check if `self` is a prime number
        for j in 0..(self.values.len()) {
            let k = self.values[j];
            if k > 0 {
                let idx = j + k as usize + 1;
                // signed to unsigned cast is twos complement,
                // so overflow would lead to a huge number which would fail the test
                if !(idx < self.n as usize * 2 && self.values.get(idx).map_or(true, |&v| v == -k)) {
                    return false;
                }
            }
        }

        true
    }

    fn next_steps(&self) -> Self::Steps {
        ((-self.n)..=self.n)
            .filter(|&v| v != 0 && !self.values.contains(&v))
            .collect()
    }

    fn next_states(&self) -> Vec<Self> {
        let mut res = Vec::new();
        for v in (-self.n)..=self.n {
            if v != 0 && !self.values.contains(&v) {
                let mut values = self.values.clone();
                values.push(v);
                res.push(Self { n: self.n, values })
            }
        }

        res
    }

    fn apply_step(&self, step: Self::Step) -> Self {
        let mut values = self.values.clone();
        values.push(step);
        Self { n: self.n, values }
    }
}

/// solves the langford pairs problem using algorithm L described on page 7 of pre-fascicle 5B.
///
/// While there are still some ways to improve this algorithm,
/// it is still a lot faster than using `b` or `w` with `LangfordPairsBrute`.
pub fn l(n: usize) -> Vec<Vec<isize>> {
    let mut results = Vec::new();

    let mut sequence = vec![0; n * 2];
    let mut position = 0;

    // A circular linked list with the length `n + 1` containing all currently free values.
    // Iterating through this list can simply be done with `ptr = unused_values[ptr]`.
    //
    // The initial condition is `[1, 2, .., n, 0]`. If we were to remove the value `2`,
    // this list is updated to `[1, 3, 3, 4, .., n, 0]`.
    let mut unused_values = (1..=n).collect::<Vec<_>>();
    unused_values.push(0);

    let mut undo = vec![0; n * 2];
    let mut ptr = 0;

    loop {
        while unused_values[ptr] != 0 && position + unused_values[ptr] + 1 < sequence.len() {
            // check if the current value and its inverse can be inserted,
            // update `ptr` to point to the nesequencet value otherwise
            if sequence[position + unused_values[ptr] + 1] == 0 {
                // insert both the value at the current position and
                // the negative value at the right offset
                sequence[position] = unused_values[ptr] as isize;
                sequence[position + unused_values[ptr] + 1] = -(unused_values[ptr] as isize);

                // skip the used value from now on from now on
                // update `undo` to allow for a quick backtrack
                undo[position] = ptr;
                unused_values[ptr] = unused_values[unused_values[ptr]];

                // go one level deeper, and reset `ptr`
                // `unused_values[0]` always points to the lowest available number
                ptr = 0;
                position += 1;

                // Check if there are no more available numbers,
                // as this means that we have used all of them and
                // thereby found a solution. We do not have to manually undo
                // this step in case we found a solution, as `unused_values[ptr] == 0`
                // automatically breaks the inner loop.
                //
                // Skip all already filled positions otherwise.
                if unused_values[ptr] == 0 {
                    results.push(sequence.clone());
                } else {
                    while sequence[position] < 0 {
                        position += 1;
                    }
                }
            } else {
                ptr = unused_values[ptr];
            }
        }

        if position != 0 {
            // set `position` to point to the last positive number
            position -= 1;
            while sequence[position] < 0 {
                position -= 1;
            }

            // remove the item at `position` from `sequence`
            let removed_value = sequence[position] as usize;
            sequence[position] = 0;
            sequence[position + removed_value + 1] = 0;

            // add the removed value back into `unused_values`
            // this can simply be done by updating the target of the
            // previous unused variable
            //
            // let's say that we previously used `2`, `3` and want to undo `3`.
            // This means that `1`, which previously pointed at `4`, has to point at `3` again.
            // [1, 4, 3, 4, 5, 0] -> [1, 2, 3, 4, 5, 0]
            unused_values[undo[position]] = removed_value;

            // update pointer to point at the free value after the one we have just
            // removed from `sequence`.
            ptr = removed_value;
        } else {
            return results;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{b, w};

    #[test]
    fn four() {
        let solutions = b(LangfordPairsBrute::new(4), 8);
        assert_eq!(solutions.len(), 2);
        assert!(solutions
            .iter()
            .any(|s| s.values == [2, 3, 4, -2, 1, -3, -1, -4]));
        assert!(solutions
            .iter()
            .any(|s| s.values == [4, 1, 3, -1, 2, -4, -3, -2]));

        let solutions = w(LangfordPairsBrute::new(4), 8);
        assert_eq!(solutions.len(), 2);
        assert!(solutions
            .iter()
            .any(|s| s.values == [2, 3, 4, -2, 1, -3, -1, -4]));
        assert!(solutions
            .iter()
            .any(|s| s.values == [4, 1, 3, -1, 2, -4, -3, -2]));

        let solutions = l(4);
        assert_eq!(solutions.len(), 2);
        assert!(solutions.iter().any(|s| s == &[2, 3, 4, -2, 1, -3, -1, -4]));
        assert!(solutions.iter().any(|s| s == &[4, 1, 3, -1, 2, -4, -3, -2]));
    }

    #[test]
    fn count() {
        (1..12)
            .map(|v| l(v).len())
            .zip(&[0, 0, 2, 2, 0, 0, 52, 300, 0, 0, 35584])
            .for_each(|(a, &b)| assert_eq!(a, b));
    }
}
