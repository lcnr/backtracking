pub mod estimate;
pub mod langford;
pub mod queens;

use std::ops::Range;

/// A required set of methods needed for the generic backtracking algorithms.
///
/// For every implemented problem in this project, there is at least one implementation of this trait.
pub trait Sequence {
    type Step;
    type Steps: IntoIterator<Item = Self::Step>;

    /// Checks if this sequence satisfies its condition.
    ///
    /// This function can assume that the  parent of `self` satisfied this condition.
    fn satisfies_condition(&self) -> bool;

    /// generates all possible next steps at this current state.
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
    /// this function will only be called if `self.satisfies_condition() == true`.
    fn apply_step(&self, step: Self::Step) -> Self;
}

/// The error of `RevertableSequence::next_step` once all steps have been tried
pub struct ExhaustedSequence;

/// A more complex version of the `Sequence` trait, used to unify all implemented algorithms.
pub trait RevertableSequence {
    /// applies the next untried step at the current state of the sequence
    /// in case there is no untried step, this returns an error and does not modify itself.
    fn next_step(&mut self) -> Result<(), ExhaustedSequence>;

    /// checks if the current sequence is valid under the condition that the
    /// previous state of `self` was valid.
    fn is_valid(&self) -> bool;

    /// undoes the last applied step
    fn undo(&mut self);
}

pub struct Simple<T: Sequence> {
    states: Vec<(T, <T::Steps as IntoIterator>::IntoIter)>,
}

impl<T: Sequence> Simple<T> {
    pub fn new(initial: T) -> Self {
        let steps = initial.next_steps();
        Self {
            states: vec![(initial, steps.into_iter())],
        }
    }
}

impl<T: Sequence> RevertableSequence for Simple<T> {
    /// applies the next untried step at the current state of the sequence
    fn next_step(&mut self) -> Result<(), ExhaustedSequence> {
        if let Some((state, steps)) = self.states.last_mut() {
            if let Some(step) = steps.next() {
                let next_state = state.apply_step(step);
                let next_steps = next_state.next_steps().into_iter();
                self.states.push((next_state, next_steps));
                return Ok(());
            }
        }

        Err(ExhaustedSequence)
    }

    /// checks if the current sequence is valid under the condition that the
    /// previous state of `self` was valid.
    fn is_valid(&self) -> bool {
        if let Some((state, _steps)) = self.states.last() {
            state.satisfies_condition()
        } else {
            true
        }
    }

    /// undoes the last applied step
    fn undo(&mut self) {
        self.states.pop();
    }
}

/// Recursively solves a backtracking problem using a recursive algorithm
///
/// This is functionally equivalent to algorithm `l`
pub fn recursive<T: Sequence>(current: T, n: usize) -> Vec<T> {
    if current.satisfies_condition() {
        // if we are at the desired depth `n`, return the current element
        // otherwise we return valid sequences containing this partial sequence
        if n == 0 {
            vec![current]
        } else {
            let mut results = Vec::new();

            for step in current.next_steps() {
                results.append(&mut recursive(current.apply_step(step), n - 1));
            }

            results
        }
    } else {
        Vec::new()
    }
}

/// Solves every backtracking problem using the basic algorithm.
///
/// Searches for a sequence of n states for which the `state.satisfies_condition()` is true.
pub fn b<T: Sequence>(initial: T, n: usize) -> Vec<T> {
    if !initial.satisfies_condition() {
        return Vec::new();
    } else if n == 0 {
        return vec![initial];
    }

    // all sequences of length n which satisfy the condition
    let mut results = Vec::new();

    // the current sequence, starts with just the initial state
    let mut states = Vec::new();

    let steps = initial.next_steps().into_iter();
    states.push((initial, steps));

    // run while there is still a state with an unchecked possible next step
    while let Some((state, steps)) = states.last_mut() {
        // take the next unchecked possible step of the current state,
        // in case there are no unchecked steps left, simply discard the current state
        // as all possible sequences have already been tried.
        if let Some(step) = steps.next() {
            // compute the result of this step
            let next_state = state.apply_step(step);
            // does this new state still satisfy the condition,
            // if not we can simply discard it
            if next_state.satisfies_condition() {
                // if the sequence is already n elements long,
                // it is correct and can be added to results.
                // Otherwise we push it onto the stack.
                if states.len() < n {
                    let next_steps = next_state.next_steps().into_iter();
                    states.push((next_state, next_steps));
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
    if !initial.satisfies_condition() {
        return Vec::new();
    } else if n == 0 {
        return vec![initial];
    }

    // initial is the only possible sequence of len `n`
    if n == 0 {
        return vec![initial];
    }

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

const BASE: u32 = 10;

/// a right truncatable prime is a prime number where any number of its trailing digits
/// can be removed and the resulting number is still a prime.
///
/// This means that 2339 is a right truncatable prime,
/// as 233, 23 and 2 are all primes themselves.
///
/// 19 is not a right truncatable prime, as 9 is divisible by 3.
#[derive(Clone)]
pub struct RightTruncatablePrime(pub u32);

impl Sequence for RightTruncatablePrime {
    type Step = u32;
    type Steps = Range<Self::Step>;

    fn satisfies_condition(&self) -> bool {
        let value = self.0;
        // check if `self` is a prime number
        if value < 2 {
            // accept 0 even if it is not actually prime
            // as we need an valid initial state
            value == 0
        } else {
            for i in 2..value {
                if value % i == 0 {
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
        Self(self.0 * BASE + step)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncatable() {
        let recursive = recursive(RightTruncatablePrime(0), 4);
        let b = b(RightTruncatablePrime(0), 4);
        let w = w(RightTruncatablePrime(0), 4);

        // there are 16 right truncatable primes with length 4
        assert_eq!(recursive.len(), 16);
        assert_eq!(b.len(), 16);
        assert_eq!(w.len(), 16);
        // one of which is 7393
        assert!(recursive.iter().any(|p| p.0 == 7393));
        assert!(b.iter().any(|p| p.0 == 7393));
        assert!(w.iter().any(|p| p.0 == 7393));
    }
}
