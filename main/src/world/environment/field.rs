use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    usize,
};

use crate::consts::ENV_SIDE_CELLS;

#[derive(Serialize, Deserialize, Resource, Copy, Clone)]
pub struct Field<T, const N: usize>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    #[serde(with = "BigArray")]
    pub space: [T; N],
    pub side_len: usize,
}

impl<T, const N: usize> Index<usize> for Field<T, N>
where
    T: Serialize + for<'a> Deserialize<'a> + Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.space[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Field<T, N>
where
    T: Serialize + for<'a> Deserialize<'a> + Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.space[index]
    }
}

impl<T, const N: usize> Field<T, N>
where
    T: Serialize + for<'a> Deserialize<'a> + Copy,
{
    pub fn get(&self, x: isize, y: isize) -> T {
        return self.space[self.get_i(x, y)];
    }
    pub fn get_mut<'a>(&'a mut self, x: isize, y: isize) -> &'a mut T {
        return &mut self.space[self.get_i(x, y)];
    }

    pub fn set(&mut self, x: isize, y: isize, value: T) {
        self.space[self.get_i(x, y)] = value;
    }

    pub fn from_element(val: T) -> Self {
        return Self {
            space: [val; N],
            side_len: ENV_SIDE_CELLS,
        };
    }

    pub fn from_array(array: [T; N]) -> Self {
        let side_len = (N as f32).sqrt();

        return Self {
            space: array,
            side_len: side_len as usize,
        };
    }

    fn get_i(&self, x: isize, y: isize) -> usize {
        let l = self.side_len as isize;

        return (x.rem_euclid(l) + y.rem_euclid(l) * l) as usize;
    }
}
// impl<const N: usize> Field<f32, N> {
//     pub fn get(&self, x: isize, y: isize) -> f32 {
//         return self.space[self.get_i(x, y)];
//     }
//     pub fn get_mut<'a>(&'a mut self, x: isize, y: isize) -> &'a mut f32 {
//         return &mut self.space[self.get_i(x, y)];
//     }

//     pub fn set(&mut self, x: isize, y: isize, value: f32) {
//         self.space[self.get_i(x, y)] = value;
//     }

//     pub fn from_element(val: f32) -> Self {
//         return Self {
//             space: [val; N],
//             side_len: ENV_SIDE_LEN,
//         };
//     }

//     pub fn from_array(array: [f32; N]) -> Self {
//         let side_len = (N as f32).sqrt();

//         return Self {
//             space: array,
//             side_len: side_len as usize,
//         };
//     }

//     fn get_i(&self, x: isize, y: isize) -> usize {
//         let l = self.side_len as isize;

//         return (x.rem_euclid(l) + y.rem_euclid(l) * l) as usize;
//     }
// }
impl<const N: usize> Debug for Field<f32, N> {
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
    use crate::world::environment::{accessor_trait::Env, field::Field};

    fn gen_field() -> Field<f32, 9> {
        Field::<f32, 9>::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0])
    }

    #[test]
    fn y_coord_get_value() {
        let f = gen_field();

        // underflow
        assert_eq!(f.get(0, -4), 7.0);
        assert_eq!(f.get(0, -2), 4.0);

        // normal
        assert_eq!(f.get(0, 0), 1.0);
        assert_eq!(f.get(0, 2), 7.0);

        // overflow
        assert_eq!(f.get(0, 4), 4.0);
        assert_eq!(f.get(0, 5), 7.0);
    }

    #[test]
    fn x_coord_get_value() {
        let f = gen_field();

        // underflow
        assert_eq!(f.get(-4, 0), 3.0);
        assert_eq!(f.get(-2, 0), 2.0);

        // normal
        assert_eq!(f.get(0, 0), 1.0);
        assert_eq!(f.get(2, 0), 3.0);

        // overflow
        assert_eq!(f.get(4, 0), 2.0);
        assert_eq!(f.get(5, 0), 3.0);
    }
}
