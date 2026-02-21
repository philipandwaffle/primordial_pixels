#[cfg(test)]
mod tests {
    use crate::world::environment::{
        accessor_trait::Env, field::Field, layer::replenish_convolve::ReplenishConvolve,
    };

    #[test]
    fn replenish_convolve() {
        let mut layer = ReplenishConvolve::<9, 9>::new(
            Field::<f32, 9>::from_element(1.0 / 9.0),
            1.0,
            5.0,
            0.25,
        );

        assert_eq!(layer[4], 0.0);

        layer.update(1.0);
        assert_eq!(layer[4], 0.25);
    }

    #[test]
    fn replenish_convolve_delta() {
        let mut layer = ReplenishConvolve::<9, 9>::new(
            Field::<f32, 9>::from_array([1.0 / 9.0; 9]),
            1.0,
            5.0,
            0.25,
        );

        assert_eq!(layer[4], 0.0);
        layer.update(1.0);

        assert_eq!(layer[4], 0.25);
        let mut delta = 0.1;
        layer.delta(1, 1, &mut delta);
        println!("{}, {}", delta, layer[4]);
    }
}
