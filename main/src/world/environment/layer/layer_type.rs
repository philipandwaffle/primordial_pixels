use std::ops::Index;

use my_derive::ConfigTag;
use serde::{Deserialize, Serialize};

use crate::{
    config::{config::Layer as LayerConfig, config_tag::ConfigTag},
    consts::{ENV_CELLS, KERNEL_CELLS},
    world::environment::{
        accessor_trait::Env,
        layer::{
            convolve::Convolve, periodic_replenish_convolve::PeriodicReplenishConvolve,
            replenish::Replenish, replenish_convolve::ReplenishConvolve,
        },
    },
};

#[derive(ConfigTag, Clone, Copy, Serialize, Deserialize)]
pub enum LayerType<const N: usize, const KN: usize> {
    Replenish(Replenish<N>),
    Convolve(Convolve<N, KN>),
    ReplenishConvolve(ReplenishConvolve<N, KN>),
    PeriodicReplenishConvolve(PeriodicReplenishConvolve<N, KN>),
}
impl<const N: usize, const KN: usize> Index<usize> for LayerType<N, KN> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            LayerType::Replenish(l) => &l[index],
            LayerType::Convolve(l) => &l[index],
            LayerType::ReplenishConvolve(l) => &l[index],
            LayerType::PeriodicReplenishConvolve(l) => &l[index],
        }
    }
}
impl<const N: usize, const KN: usize> Env<N> for LayerType<N, KN> {
    fn field(&self) -> &crate::world::environment::field::Field<f32, N> {
        match self {
            LayerType::Replenish(l) => l.field(),
            LayerType::Convolve(l) => l.field(),
            LayerType::ReplenishConvolve(l) => l.field(),
            LayerType::PeriodicReplenishConvolve(l) => l.field(),
        }
    }

    fn field_mut(&mut self) -> &mut crate::world::environment::field::Field<f32, N> {
        match self {
            LayerType::Replenish(l) => l.field_mut(),
            LayerType::Convolve(l) => l.field_mut(),
            LayerType::ReplenishConvolve(l) => l.field_mut(),
            LayerType::PeriodicReplenishConvolve(l) => l.field_mut(),
        }
    }

    fn get(&self, x: isize, y: isize) -> f32 {
        match self {
            LayerType::Replenish(replenish_layer) => replenish_layer.get(x, y),
            LayerType::Convolve(convolve_layer) => convolve_layer.get(x, y),
            LayerType::ReplenishConvolve(replenish_convolve) => replenish_convolve.get(x, y),
            LayerType::PeriodicReplenishConvolve(periodic_replenish_convolve) => {
                periodic_replenish_convolve.get(x, y)
            }
        }
    }

    fn max(&self) -> f32 {
        match self {
            LayerType::Replenish(replenish) => replenish.max(),
            LayerType::Convolve(convolve) => convolve.max(),
            LayerType::ReplenishConvolve(replenish_convolve) => replenish_convolve.max(),
            LayerType::PeriodicReplenishConvolve(periodic_replenish_convolve) => {
                periodic_replenish_convolve.max()
            }
        }
    }

    fn update(&mut self, dt: f32) {
        match self {
            LayerType::Replenish(replenish_layer) => replenish_layer.update(dt),
            LayerType::Convolve(convolve_layer) => convolve_layer.update(dt),
            LayerType::ReplenishConvolve(replenish_convolve) => replenish_convolve.update(dt),
            LayerType::PeriodicReplenishConvolve(periodic_replenish_convolve) => {
                periodic_replenish_convolve.update(dt)
            }
        }
    }
}

impl<const N: usize, const KN: usize> From<LayerConfig<KN>> for LayerType<N, KN> {
    fn from(layer_config: LayerConfig<KN>) -> Self {
        match layer_config {
            LayerConfig::Energy {
                kernel,
                permeability,
                max,
                rate,
                on_interval,
                off_interval,
            } => LayerType::PeriodicReplenishConvolve(PeriodicReplenishConvolve::new(
                kernel,
                permeability,
                max,
                rate,
                on_interval,
                off_interval,
            )),
            LayerConfig::Decompose {
                initial_value,
                kernel,
                permeability,
                max,
            } => LayerType::Convolve(Convolve::new(initial_value, kernel, permeability, max)),
            LayerConfig::Pheromone {
                kernel,
                permeability,
                max,
            } => LayerType::Convolve(Convolve::new(0.0, kernel, permeability, max)),
        }
    }
}
