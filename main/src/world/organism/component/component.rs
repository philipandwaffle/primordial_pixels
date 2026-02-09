use std::collections::VecDeque;

use bevy::{
    asset::Handle,
    ecs::{component::Component, entity::Entity, system::Commands},
    log::error,
    math::Vec2,
    sprite_render::ColorMaterial,
};

use crate::{
    assets::handles::{Handles, MatKey},
    config::config::Node as TransputConfig,
    consts::{MAX_MUSCLE_LEN, MIN_MUSCLE_LEN},
    world::organism::{
        body::Body,
        brain::Brain,
        node_type::NodeType,
        organism::Organism,
        seed::Seed,
        stats::{StaticStats, VariableStats},
        transput::{Transput, TransputUtil},
        util_trait::OrganismAccessor,
    },
};





