use crate::Sequence;

use rand::{seq::IteratorRandom, thread_rng};

use std::time::{Duration, Instant};

/// Tries to build a random sequence of length `n` without any backtracking,
/// returns the estimated time actual backtracking would take for this sequence.
///
/// This function only tries 1 path, so the results are highly unstable and might
/// not always be representative. For a better estimate use `repeated_estimate`
/// with many `repetitions`.
pub fn e<T: Sequence>(initial: T, n: usize) -> Duration {
    let mut current = initial;

    let mut estimated_steps = 1;
    let mut total_length = Duration::from_secs(0);

    for _ in 0..n - 1 {
        // time the duration of calculating all valid next states
        let now = Instant::now();
        let mut next_states = current.next_states();
        next_states.retain(|state| state.satisfies_condition());
        total_length += now.elapsed() * estimated_steps;

        // we expect our current branch to be representative of all
        // possible branches
        estimated_steps *= next_states.len() as u32;

        // choose a random next state
        if let Some(next) = next_states.into_iter().choose(&mut thread_rng()) {
            current = next;
        } else {
            break;
        }
    }

    total_length
}

/// tries to estimate the total time needed to solve the given backtracking problem
/// by repeatedly calling `e` and calculating the average
pub fn repeated_estimate<T: Sequence + Clone>(initial: T, n: usize, repetitions: u32) -> Duration {
    let mut total_duration = Duration::from_secs(0);

    for _ in 0..repetitions {
        total_duration += e(initial.clone(), n);
    }

    total_duration / repetitions
}
