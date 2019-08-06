use crate::Sequence;

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
                let idx = j as isize + k + 1;
                // signed to unsigned cast is twos complement,
                // so overflow would lead to a huge number which would fail the test
                if !(idx < self.n * 2 && self.values.get(idx as usize).map_or(true, |&v| v == -k)) {
                    return false;
                }
            }
        }

        true
    }

    fn next_steps(&self) -> Self::Steps {
        ((-self.n)..=self.n).filter(|&v| v != 0 && !self.values.contains(&v)).collect()
    }

    fn next_states(&self) -> Vec<Self> {
        let mut res = Vec::new();
        for v in (-self.n)..=self.n {
            if v != 0 && !self.values.contains(&v) {
                let mut values = self.values.clone();
                values.push(v);
                res.push(Self {
                    n: self.n,
                    values,
                })
            }
        }

        res
    }

    fn apply_step(&self, step: Self::Step) -> Self {
        let mut values = self.values.clone();
        values.push(step);
        Self {
            n: self.n,
            values
        }
    }
}

struct ValueStorage {
    list: Vec<usize>,
    previous: usize,
    curr: usize,
}

impl ValueStorage {
    fn new(n: usize) -> Self {
        // a linked list containing all the currently still unused values
        // starts as [1, .., n, 0] with a pointer on `unused_values[0]`
        // in case `2` and `5` are used with `n == 7`, this would be
        // [1, 3, _, 4, 6, _, 7, 0]. In this state both index `2` and `5` are unreachable.
        let mut list = (1..=n).collect::<Vec<_>>();
        list.push(0);

        let previous = 0;
        let curr = list[previous];

        Self {
            list,
            previous,
            curr,
        }
    }

    /// The current value
    fn value(&self) -> usize {
        self.list[self.curr]
    }
}

/// solves the langford pairs problem using algorithm L described on page 7 of pre-fascicle 5B.
/// 
/// While there are still some ways to improve this algorithm,
/// it is still a lot faster than using `b` or `w` with `LangfordPairsBrute`.
pub fn l(n: usize) -> Vec<Vec<isize>> {
    let mut results = Vec::new();

    let mut x = vec![0; n * 2];

    
    let mut unused_values = (1..=n).collect::<Vec<_>>();
    unused_values.push(0);

    let mut undo = vec![0; n * 2];

    let mut position = 0;
    let mut current = 0;
    let mut value = unused_values[current];

    enum State {
        L3,
        L4,
        L5,
    }

    let mut state = State::L3;
    
    loop {
        while value != 0 && position + value + 1 < x.len() {
            if x[position + value + 1] == 0 {
                x[position + value + 1] = -(value as isize);
                x[position] = value as isize;
                undo[position] = current;

                // skip `value` from now on
                // and go one level deeper
                unused_values[current] = unused_values[value];
                current = 0;
                position += 1;

                value = unused_values[current];

                if value == 0 {
                    results.push(x.clone());
                } else {
                    while x[position] < 0 {
                        position += 1;
                    }
                }
            } else {
                current = value;
                value = unused_values[value];
            }
        }

        if position == 0 {
            return results
        } else {
            position -= 1;
            while x[position] < 0 {
                position -= 1;
            }

            value = x[position] as usize;
            x[position] = 0;
            x[position + value + 1] = 0;
            
            unused_values[current] = value;
            current = value;
            value = unused_values[value];
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
        assert!(solutions.iter().any(|s| s.values == [2, 3, 4, -2, 1, -3, -1, -4]));
        assert!(solutions.iter().any(|s| s.values == [4, 1, 3, -1, 2, -4, -3, -2]));

        let solutions = w(LangfordPairsBrute::new(4), 8);
        assert_eq!(solutions.len(), 2);
        assert!(solutions.iter().any(|s| s.values == [2, 3, 4, -2, 1, -3, -1, -4]));
        assert!(solutions.iter().any(|s| s.values == [4, 1, 3, -1, 2, -4, -3, -2]));

        let solutions = l(4);
        assert_eq!(solutions.len(), 2);
        assert!(solutions.iter().any(|s| s == &[2, 3, 4, -2, 1, -3, -1, -4]));
        assert!(solutions.iter().any(|s| s == &[4, 1, 3, -1, 2, -4, -3, -2]));
    }

    #[test]
    fn count() {
        (1..12).map(|v| l(v).len()).zip(&[0, 0, 2, 2, 0, 0, 52, 300, 0, 0, 35584]).for_each(|(a, &b)| assert_eq!(a, b));
    }
}