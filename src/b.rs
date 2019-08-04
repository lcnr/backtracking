//! the core algorithm B (Basic Backtrack).

pub trait Sequence {
    type Step;

    /// does this sequence satisfy its condition.
    fn satisfies_condition(&self) -> bool;

    /// generates the current domain.
    fn next_steps(&self) -> Vec<Self::Step>;

    /// applies a step to self
    /// 
    /// `self.apply_step(x).satisfies_condition()` must only be true if
    /// `self.satisfies_condition()` is true
    fn apply_step(&self, step: Self::Step) -> Self;
}

struct State<T: Sequence> {
    value: T,
    unchecked_steps: Vec<T::Step>,
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

    // the current sequence, simply starts as the start condition
    let mut states = Vec::new();
    states.push(State::new(initial));

    // run while there is still a state with an unchecked possible next step
    while let Some(state) = states.last_mut() {
        // take the next unchecked possible step of the current state,
        // in case there are no unchecked steps left, simply discard the current state
        // as all possible sequences have already been tried.
        if let Some(step) = state.unchecked_steps.pop() {
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

#[cfg(test)]
mod tests {
    use super::*;

    type RightTruncatablePrime = u32;
    
    impl Sequence for RightTruncatablePrime {
        type Step = char;

        fn satisfies_condition(&self) -> bool {
            // check if `self` is a prime number
            if *self < 2 {
                false
            } else {
                for i in 2..*self {
                    if self % i == 0 {
                        return false
                    }
                }
                true
            }
        }

        fn next_steps(&self) -> Vec<Self::Step> {
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        }

        fn apply_step(&self, step: Self::Step) -> Self {
            format!("{}{}", self, step).parse().unwrap()
        }
    }

    #[test]
    fn truncatable() {
        let primes = b(0, 4);

        // there are 16 right truncatable primes with length 4
        assert_eq!(primes.len(), 16);
        // one of which is 7393
        assert!(primes.iter().any(|&p| p == 7393));
    }
}