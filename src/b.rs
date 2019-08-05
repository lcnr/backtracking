//! the core algorithm B (Basic Backtrack).

pub trait Sequence {
    type Step;
    type Steps: Iterator<Item = Self::Step>;

    /// does this sequence satisfy its condition.
    fn satisfies_condition(&self) -> bool;

    /// generates the current domain.
    fn next_steps(&self) -> Self::Steps;

    /// generates all possible next states.
    fn next_states(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        self.next_steps()
            .into_iter()
            .map(|s| self.apply_step(s))
            .collect()
    }

    /// applies a `step` to `self`, returning the resulting sequence.
    ///
    /// `self.apply_step(x).satisfies_condition()` must only be true if
    /// `self.satisfies_condition()` is true.
    fn apply_step(&self, step: Self::Step) -> Self;
}

struct State<T: Sequence> {
    value: T,
    unchecked_steps: T::Steps,
}

impl<T: Sequence> State<T> {
    pub fn new(value: T) -> Self {
        let unchecked_steps = value.next_steps();

        Self {
            value,
            unchecked_steps,
        }
    }
}

/// Solves every backtracking problem using the basic algorithm.
///
/// Searches for a sequence of n states for which the `state.satisfies_condition()` is true.
pub fn b<T: Sequence>(initial: T, n: usize) -> Vec<T> {
    // all sequences of length n which satisfy the condition
    let mut results = Vec::new();

    // the current sequence, starts with just the initial state
    let mut states = Vec::new();
    states.push(State::new(initial));

    // run while there is still a state with an unchecked possible next step
    while let Some(state) = states.last_mut() {
        // take the next unchecked possible step of the current state,
        // in case there are no unchecked steps left, simply discard the current state
        // as all possible sequences have already been tried.
        if let Some(step) = state.unchecked_steps.next() {
            // compute the result of this step
            let next_state = state.value.apply_step(step);
            // does this new state still satisfy the condition,
            // if not we can simply discard it
            if next_state.satisfies_condition() {
                // if the sequence is already n elements long,
                // it is correct and can be added to results.
                // Otherwise we push it on the stack.
                if states.len() < n {
                    states.push(State::new(next_state));
                } else {
                    results.push(next_state);
                }
            }
        } else {
            states.pop();
        }
    }

    results
}

/// Searches for a sequence of n states for which the `state.satisfies_condition()` is true using
/// using the Walker's backtracking algorithm.
///
/// This is mostly identical to `b` but may have a slightly different performance, depending
/// on the size of `T` and `T::Step` and if there is some work in `fn next_states` which can
/// be reused.
pub fn w<T: Sequence>(initial: T, n: usize) -> Vec<T> {
    // all sequences of length n which satisfy the condition
    let mut results = Vec::new();

    // the current sequence, unlike in `b`, this now contains all relevant unchecked states
    // while `states` in algorithm `b` contains already check states together with their unchecked steps.
    let mut unchecked_states = Vec::new();
    unchecked_states.push(initial.next_states());

    // run while there is still an unchecked state
    while let Some(states) = unchecked_states.last_mut() {
        // take the next unchecked state of the current depth
        // and simply go back one step if no states are left.
        if let Some(state) = states.pop() {
            // does this state still satisfy the condition,
            // if not we can simply discard it
            if state.satisfies_condition() {
                // if the sequence is already n elements long,
                // it is correct and can be added to results.
                // Otherwise we push it on the stack.
                if unchecked_states.len() < n {
                    unchecked_states.push(state.next_states());
                } else {
                    results.push(state);
                }
            }
        } else {
            unchecked_states.pop();
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::ops::Range;

    const BASE: u32 = 10;

    /// a right truncatable prime is a prime number where any number of its trailing digits
    /// can be removed and the resulting number is still a prime.
    ///
    /// This means that 2339 is a right truncatable prime,
    /// as 233, 23 and 2 are all primes themselves.
    ///
    /// 19 is not a right truncatable prime, as 9 is divisible by 3.
    type RightTruncatablePrime = u32;

    impl Sequence for RightTruncatablePrime {
        type Step = u32;
        type Steps = Range<Self::Step>;

        fn satisfies_condition(&self) -> bool {
            // check if `self` is a prime number
            if *self < 2 {
                false
            } else {
                for i in 2..*self {
                    if self % i == 0 {
                        return false;
                    }
                }
                true
            }
        }

        fn next_steps(&self) -> Self::Steps {
            1..BASE
        }

        fn apply_step(&self, step: Self::Step) -> Self {
            self * BASE + step
        }
    }

    #[test]
    fn truncatable() {
        let b = b(0, 4);
        let w = w(0, 4);

        // there are 16 right truncatable primes with length 4
        assert_eq!(b.len(), 16);
        assert_eq!(w.len(), 16);
        // one of which is 7393
        assert!(b.iter().any(|&p| p == 7393));
        assert!(w.iter().any(|&p| p == 7393));
    }
}
