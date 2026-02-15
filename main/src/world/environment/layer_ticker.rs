use avian2d::parry::utils::hashmap::HashMap;
use bevy::ecs::resource::Resource;

use crate::{util::ticker::Ticker, world::environment::layer::layer_key::LayerKey};

#[derive(Resource)]
pub struct LayerTicker {
    tickers: HashMap<LayerKey, Ticker>,
}

