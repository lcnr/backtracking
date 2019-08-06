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

    fn apply_step(&self, step: Self::Step) -> Self {
        let mut values = self.values.clone();
        values.push(step);
        Self {
            n: self.n,
            values
        }
    }
}

/// solves the langford pairs problem using algorithm L described on page 7 of pre-fascicle 5B.
/// 
/// While there are still some ways to improve this algorithm,
/// it is still a lot faster than using `b` or `w` with `LangfordPairsBrute`.
pub fn l(n: usize) -> Vec<Vec<isize>> {
    let mut results = Vec::new();

    let mut x = vec![0; n * 2];
    let mut y = vec![0; n * 2];

    // a linked list containing all the currently still unused values
    // starts as [1, .., n, 0] with a pointer on `unused_values[0]`
    // in case `2` and `5` are used with `n == 7`, this would be
    // [1, 3, _, 4, 6, _, 7, 0]. In this state both index `2` and `5` are unreachable.
    let mut unused_values = (1..=n).collect::<Vec<_>>();
    unused_values.push(0);

    let mut k = 0;
    let mut l = 0;
    let mut j = 0;

    enum State {
        L2,
        L3,
        L4,
        L5,
    }

    let mut state = State::L2;
    
    loop {
        state = match state {
            State::L2 => {
                k = unused_values[0];
                if k == 0 {
                    results.push(x.clone());
                    State::L5
                } else {
                    j = 0;
                    while x[l] < 0 {
                        l += 1;
                    }
                    State::L3
                }
            }
            State::L3 => {
                if let Some(v) = x.get_mut(l + k + 1) {
                    if *v == 0 {
                        *v = -(k as isize);
                        x[l] = k as isize;
                        y[l] = j;

                        // skip `k` from now on
                        unused_values[j] = unused_values[k];
                        l += 1;
                        State::L2
                    } else {
                        State::L4
                    }
                } else {
                    State::L5
                }
            }
            State::L4 => {
                // get the next unused value
                j = k;
                k = unused_values[j];
                if k == 0 {
                    State::L5
                } else {
                    State::L3
                }
            }
            State::L5 => {
                if let Some(new_l) = l.checked_sub(1) {
                    l = new_l;
                    while x[l] < 0 {
                        l -= 1;
                    }

                    k = x[l] as usize;
                    x[l] = 0;
                    x[l + k + 1] = 0;
                    j = y[l];
                    unused_values[j] = k;
                    State::L4
                } else {
                    return results
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::b;

    #[test]
    fn four() {
        let solutions = b(LangfordPairsBrute::new(4), 8);
        assert_eq!(solutions.len(), 2);
        assert!(solutions.iter().any(|s| s.values == [2, 3, 4, -2, 1, -3, -1, -4]));
        assert!(solutions.iter().any(|s| s.values == [4, 1, 3, -1, 2, -4, -3, -2]));

        let solutions = l(4);
        assert_eq!(solutions.len(), 2);
        assert!(solutions.iter().any(|s| s == &[2, 3, 4, -2, 1, -3, -1, -4]));
        assert!(solutions.iter().any(|s| s == &[4, 1, 3, -1, 2, -4, -3, -2]));
    }
}