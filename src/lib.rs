extern crate rand;

use rand::distributions::{IndependentSample, Range};

pub struct MetropolisHastingsSampler<T> where T : Clone
{
    relative_probability : fn(&T, &T) -> f64,
    generator : fn(&T) -> T,
    state : T,
}

impl<T> MetropolisHastingsSampler<T> where T : Clone
{
    pub fn new(initial_state : T,
           relative_probability : fn(&T, &T) -> f64,
           generator : fn(&T) -> T)
               -> MetropolisHastingsSampler<T>
    {
        MetropolisHastingsSampler {
            relative_probability : relative_probability,
            generator : generator,
            state : initial_state,
        }
    }
}

impl<T> Iterator for MetropolisHastingsSampler<T> where T : Clone {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut rng = rand::thread_rng();

        let candidate = (self.generator)(&self.state);
        let relative_prob = (self.relative_probability)(&candidate, &self.state);
        let uniform0to1 = Range::new(0.0, 1.0);
        if uniform0to1.ind_sample(&mut rng) < relative_prob {
            self.state = candidate
        }
        Some(self.state.clone())
    }
}

//tests are kind of spotty
#[cfg(test)]
mod tests {
    #[test]
    fn print_biased_random_walk() {
        let mhs = ::MetropolisHastingsSampler::new(0, |_ : &i32, _ : &i32| {0.5}, |x : &i32| {x+1});
        for i in mhs.take(20) {
             println!("> {}", i);
        }
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
