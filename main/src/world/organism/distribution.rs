use rand::{Rng, rngs::ThreadRng};

pub trait Distribution<const NUM_TYPES: usize> {
    fn normalise(&mut self);
    fn get_index(&self, rng: &mut ThreadRng) -> usize;
    fn get_bounds(&self) -> [f32; NUM_TYPES];
}

impl<const NUM_TYPES: usize> Distribution<NUM_TYPES> for [f32; NUM_TYPES] {
    fn normalise(&mut self) {
        let total = self.iter().sum::<f32>();
        self.iter_mut().for_each(|w| *w /= total);
    }

    fn get_index(&self, rng: &mut ThreadRng) -> usize {
        let val = rng.random::<f32>();
        let bounds = self.get_bounds();

        for i in 0..NUM_TYPES {
            if val < bounds[i] {
                return i;
            }
        }

        unreachable!(
            "Value ({val}) picked for mutation distribution doesn't lie within bounds {bounds:?}"
        );
    }

    fn get_bounds(&self) -> [f32; NUM_TYPES] {
        let mut bounds = [0.0; NUM_TYPES];
        bounds[0] = self[0];

        for i in 1..NUM_TYPES - 1 {
            bounds[i] = bounds[i - 1] + self[i];
        }
        bounds[NUM_TYPES - 1] = 1.0;
        return bounds;
    }
}
