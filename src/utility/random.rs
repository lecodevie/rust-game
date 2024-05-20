use std::ops::RangeInclusive;
use rand::Rng;

pub fn generated_by<T>(range: RangeInclusive<T>) -> T
    where T: PartialOrd + rand::distributions::uniform::SampleUniform,
{
    rand::thread_rng().gen_range(range)
}