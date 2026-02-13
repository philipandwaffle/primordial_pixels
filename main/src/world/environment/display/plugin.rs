use bevy::{
    app::{Plugin, PostStartup, Update},
    asset::Assets,
    ecs::{
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    math::vec2,
    sprite_render::{ColorMaterial, MeshMaterial2d},
    time::Time,
};

use crate::{
    assets::handles::Handles,
    consts::{ENV_SIDE_CELLS, KERNEL_CELLS, NUM_COLORS, ENV_CELLS},
    util::ticker::Ticker,
    world::environment::{
        accessor_trait::Env,
        display::{bundle::DisplayCellBundle, component::DisplayCell, resource::Display},
        environment::Environment,
    },
};

pub struct DisplayPlugin {
    update_interval: f32,
}
impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let mut ticker = Ticker::new(self.update_interval);
        app.add_systems(
            PostStartup,
            (Self::init_display, Self::init_display_cells).chain(),
        )
        .add_systems(
            Update,
            move |time: Res<Time>,
                  d: Res<Display>,
                  env: Res<Environment<ENV_CELLS, KERNEL_CELLS>>,
                  cells: Query<&mut MeshMaterial2d<ColorMaterial>, With<DisplayCell>>| {
                Self::update_display_cells(time, d, env, cells, &mut ticker);
            },
        );
    }
}
impl DisplayPlugin {
    pub fn new(update_interval: f32) -> Self {
        Self { update_interval }
    }

    fn init_display(mut commands: Commands, mut mats: ResMut<Assets<ColorMaterial>>) {
        commands.insert_resource(Display::new(&mut mats));
    }

    fn init_display_cells(
        mut commands: Commands,
        mut display: ResMut<Display>,
        env: Res<Environment<ENV_CELLS, KERNEL_CELLS>>,
        handles: Res<Handles>,
    ) {
        let cell_size = env.side_len / ENV_SIDE_CELLS as f32;
        let mut pos = vec2(-env.side_len * 0.5, -env.side_len * 0.5);

        for y in 0..ENV_SIDE_CELLS {
            for x in 0..ENV_SIDE_CELLS {
                let i = y * ENV_SIDE_CELLS + x;
                display.field.space[i] = commands
                    .spawn(DisplayCellBundle::new(
                        pos + cell_size * 0.5,
                        0.99 * cell_size,
                        &display,
                        &handles,
                    ))
                    .id();
                pos.x += cell_size
            }
            pos.y += cell_size;
            pos.x -= cell_size * ENV_SIDE_CELLS as f32;
        }
    }

    fn update_display_cells(
        time: Res<Time>,
        d: Res<Display>,
        env: Res<Environment<ENV_CELLS, KERNEL_CELLS>>,
        mut cells: Query<&mut MeshMaterial2d<ColorMaterial>, With<DisplayCell>>,
        ticker: &mut Ticker,
    ) {
        if !ticker.apply_dt(time.delta_secs()) {
            return;
        }

        let max = env[&d.cur_layer].max();
        for i in 0..ENV_CELLS {
            if let Ok(mut mat) = cells.get_mut(d[i]) {
                let color_i =
                    ((env[&d.cur_layer][i] / max) * (NUM_COLORS - 1) as f32).round() as usize;
                mat.0 = d.colors[color_i].clone()
            }
        }
    }
}
