//
// This is completely overkill lol
//

use rand::{
    distributions::uniform::{SampleUniform},
    rngs::SmallRng,
    Rng, RngCore, SeedableRng,
};
use std::{cmp::PartialOrd, marker::Copy};

// pub struct AnyRngGenerator<...>...
pub struct SmallRngGenerator<T: SampleUniform + PartialOrd + Copy>(T);

impl<T> NumberGenerator<T> for SmallRngGenerator<T>
where
    T: SampleUniform + PartialOrd + Copy,
{
    type Seedable = SmallRng;
}

/// A trait that can generate random numbers xd
pub trait NumberGenerator<T>
where
    T: SampleUniform + PartialOrd + Copy,
{
    type Seedable: SeedableRng + RngCore;

    // Get a set of random numbers inclusively and return the result as a vector.
    fn get_random_numbers_inclusive(n: u32, range: &std::ops::RangeInclusive<T>) -> Vec<T> {
        let mut rng = <Self::Seedable as SeedableRng>::from_entropy();
        (0..n)
            .map(move |_| rng.gen_range(range.clone()))
            .collect::<Vec<T>>()
    }
    // have other methods if needed for exclusive or more complex requirements.
}
