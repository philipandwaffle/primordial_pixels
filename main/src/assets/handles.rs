use std::collections::HashMap;

use bevy::{
    asset::{Assets, Handle},
    color::{Color, Hue},
    ecs::resource::Resource,
    math::primitives::{Circle, Rectangle, RegularPolygon},
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
};

#[derive(Eq, Hash, PartialEq)]
pub enum MeshKey {
    Circle,
    Rectangle,
    Hex,
}
#[derive(Eq, Hash, PartialEq)]
pub enum MatKey {
    Red,
    Orange,
    Yellow,
    SpringGreen,
    Green,
    OceanGreen,
    Cyan,
    SkyBlue,
    Blue,
    Purple,
    Magenta,
    Crimson,
    White,
    LightGrey,
    Grey,
    DarkGrey,
    Black,
}

#[derive(Resource)]
pub struct Handles {
    pub meshes: HashMap<MeshKey, Handle<Mesh>>,
    pub mats: HashMap<MatKey, Handle<ColorMaterial>>,
    pub colors: HashMap<MatKey, Color>,
}
impl Handles {
    pub fn new() -> Self {
        return Handles {
            meshes: HashMap::new(),
            mats: HashMap::new(),
            colors: HashMap::new(),
        };
    }

    pub fn get_mesh_handle(&self, k: &MeshKey) -> Handle<Mesh> {
        return self.meshes[k].clone();
    }
    pub fn get_mesh2d(&self, k: &MeshKey) -> Mesh2d {
        return Mesh2d(self.get_mesh_handle(k));
    }

    pub fn get_mat_handle(&self, k: &MatKey) -> Handle<ColorMaterial> {
        return self.mats[k].clone();
    }
    pub fn get_mat2d(&self, k: &MatKey) -> MeshMaterial2d<ColorMaterial> {
        return MeshMaterial2d(self.get_mat_handle(k));
    }

    pub fn insert_circle_mesh(&mut self, key: MeshKey, meshes: &mut Assets<Mesh>, radius: f32) {
        self.meshes
            .insert(key, meshes.add(Circle::new(radius)).into());
    }
    pub fn insert_rectangle_mesh(
        &mut self,
        key: MeshKey,
        meshes: &mut Assets<Mesh>,
        w: f32,
        h: f32,
    ) {
        self.meshes
            .insert(key, meshes.add(Rectangle::new(w, h)).into());
    }
    pub fn insert_hex_mesh(&mut self, key: MeshKey, meshes: &mut Assets<Mesh>, r: f32) {
        self.meshes
            .insert(key, meshes.add(RegularPolygon::new(r, 6)).into());
    }

    pub fn insert_mat(&mut self, key: MatKey, mats: &mut Assets<ColorMaterial>, color: Color) {
        self.mats.insert(key, mats.add(color).into());
    }

    pub fn setup_meshes(&mut self, meshes: &mut Assets<Mesh>) {
        self.insert_circle_mesh(MeshKey::Circle, meshes, 1.0);
        self.insert_rectangle_mesh(MeshKey::Rectangle, meshes, 1.0, 1.0);
        self.insert_hex_mesh(MeshKey::Hex, meshes, 1.0);
    }

    pub fn setup_mats(&mut self, mats: &mut Assets<ColorMaterial>) {
        let mut color = Color::hsla(0.0, 1.0, 0.5, 1.0);

        self.insert_mat(MatKey::Red, mats, color);
        self.colors.insert(MatKey::Red, color);
        color = color.rotate_hue(30.0);

        self.insert_mat(MatKey::Orange, mats, color);
        self.colors.insert(MatKey::Orange, color);
        color = color.rotate_hue(30.0);

        self.insert_mat(MatKey::Yellow, mats, color);
        self.colors.insert(MatKey::Yellow, color);
        color = color.rotate_hue(30.0);

        self.insert_mat(MatKey::SpringGreen, mats, color);
        self.colors.insert(MatKey::SpringGreen, color);
        color = color.rotate_hue(30.0);

        self.insert_mat(MatKey::Green, mats, color);
        self.colors.insert(MatKey::Green, color);
        color = color.rotate_hue(30.0);

        self.insert_mat(MatKey::OceanGreen, mats, color);
        self.colors.insert(MatKey::OceanGreen, color);
        color = color.rotate_hue(30.0);

        self.insert_mat(MatKey::Cyan, mats, color);
        self.colors.insert(MatKey::Cyan, color);
        color = color.rotate_hue(30.0);

        self.insert_mat(MatKey::SkyBlue, mats, color);
        self.colors.insert(MatKey::SkyBlue, color);
        color = color.rotate_hue(30.0);

        self.insert_mat(MatKey::Blue, mats, color);
        self.colors.insert(MatKey::Blue, color);
        color = color.rotate_hue(30.0);

        self.insert_mat(MatKey::Purple, mats, color);
        self.colors.insert(MatKey::Purple, color);
        color = color.rotate_hue(30.0);

        self.insert_mat(MatKey::Magenta, mats, color);
        self.colors.insert(MatKey::Magenta, color);
        color = color.rotate_hue(30.0);

        self.insert_mat(MatKey::Crimson, mats, color);
        self.colors.insert(MatKey::Crimson, color);

        self.insert_mat(MatKey::White, mats, Color::hsla(0.0, 0.0, 1.0, 1.0));
        self.insert_mat(MatKey::LightGrey, mats, Color::hsla(0.0, 0.0, 0.75, 1.0));
        self.insert_mat(MatKey::Grey, mats, Color::hsla(0.0, 0.0, 0.5, 1.0));
        self.insert_mat(MatKey::DarkGrey, mats, Color::hsla(0.0, 0.0, 0.25, 1.0));
        self.insert_mat(MatKey::Black, mats, Color::hsla(0.0, 0.0, 0.0, 1.0));
    }
}
