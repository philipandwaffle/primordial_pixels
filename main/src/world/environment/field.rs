use bevy::{
    log::error,
    math::{Vec2, vec2},
    prelude::Resource,
};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::{fmt::Debug, usize};

#[derive(Serialize, Deserialize, Resource, Copy, Clone)]
pub struct Field<const N: usize> {
    #[serde(with = "BigArray")]
    pub space: [f32; N],
    pub side_len: usize,
}
impl<const N: usize> Field<N> {
    pub fn from_element(val: f32) -> Self {
        let side_len = (N as f32).sqrt();
        if side_len.fract() != 0.0 {
            panic!("The number of cells must be a square");
        }

        return Self {
            space: [val; N],
            side_len: side_len as usize,
        };
    }

    pub fn from_array(array: [f32; N]) -> Self {
        let side_len = (N as f32).sqrt();

        return Self {
            space: array,
            side_len: side_len as usize,
        };
    }

    pub fn get_value(&self, x: isize, y: isize) -> f32 {
        return self.space[self.get_i(x, y)];
    }

    pub fn set_value(&mut self, x: isize, y: isize, val: f32) {
        self.space[self.get_i(x, y)] = val;
    }

    fn get_i(&self, x: isize, y: isize) -> usize {
        let l = self.side_len as isize;

        return (x.rem_euclid(l) + y.rem_euclid(l) * l) as usize;
    }
}
impl<const N: usize> Debug for Field<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.side_len;
        let mut rows = Vec::with_capacity(len);
        for x in 0..len {
            let mut row = Vec::with_capacity(len);
            for y in 0..len {
                row.push(self.space[x + y * len]);
            }
            rows.push(row);
        }

        f.debug_struct("Field").field("space", &rows).finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::world::environment::field::Field;

    fn gen_field() -> Field<9> {
        Field::<9>::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0])
    }

    #[test]
    fn y_coord_get_value() {
        let f = gen_field();

        // underflow
        assert_eq!(f.get_value(0, -4), 7.0);
        assert_eq!(f.get_value(0, -2), 4.0);

        // normal
        assert_eq!(f.get_value(0, 0), 1.0);
        assert_eq!(f.get_value(0, 2), 7.0);

        // overflow
        assert_eq!(f.get_value(0, 4), 4.0);
        assert_eq!(f.get_value(0, 5), 7.0);
    }

    #[test]
    fn x_coord_get_value() {
        let f = gen_field();

        // underflow
        assert_eq!(f.get_value(-4, 0), 3.0);
        assert_eq!(f.get_value(-2, 0), 2.0);

        // normal
        assert_eq!(f.get_value(0, 0), 1.0);
        assert_eq!(f.get_value(2, 0), 3.0);

        // overflow
        assert_eq!(f.get_value(4, 0), 2.0);
        assert_eq!(f.get_value(5, 0), 3.0);
    }
}
