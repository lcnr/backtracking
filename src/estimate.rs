use crate::Sequence;

use rand::{seq::IteratorRandom, thread_rng};

use std::time::{Duration, Instant};

pub fn e<T: Sequence>(initial: T, n: usize) -> Duration {
    let mut current = initial;

    let mut estimated_steps = 1;
    let mut total_length = Duration::from_secs(0);

    let mut depth = 0;
    while depth < n {
        let now = Instant::now();
        let mut next_states = current.next_states();
        next_states.retain(|state| state.satisfies_condition());
        total_length += now.elapsed() * estimated_steps;

        estimated_steps *= next_states.len() as u32;
        if let Some(next) = next_states.into_iter().choose(&mut thread_rng()) {
            depth += 1;
            current = next;
        } else {
            break;
        }
    }

    total_length
}

pub fn repeated_estimate<T: Sequence + Clone>(initial: T, n: usize, repetitions: u32) -> Duration {
    let mut total_duration = Duration::from_secs(0);

    for _ in 0..repetitions {
        total_duration += e(initial.clone(), n);
    }

    total_duration / repetitions
}
