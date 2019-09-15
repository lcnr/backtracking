use std::ops::Range;

use crate::Sequence;

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

    use crate::{b, recursive, w};

    #[test]
    fn truncatable() {
        let recursive = recursive(RightTruncatablePrime(0), 4);
        let b = b(RightTruncatablePrime(0), 4);
        let w = w(RightTruncatablePrime(0), 4);

        // there are 16 right truncatable primes of length 4
        assert_eq!(recursive.len(), 16);
        assert_eq!(b.len(), 16);
        assert_eq!(w.len(), 16);
        // one of which is 7393
        assert!(recursive.iter().any(|p| p.0 == 7393));
        assert!(b.iter().any(|p| p.0 == 7393));
        assert!(w.iter().any(|p| p.0 == 7393));
    }
}
