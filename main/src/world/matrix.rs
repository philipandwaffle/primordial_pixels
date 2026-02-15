use my_derive::ConfigTag;
use nalgebra::DMatrix;
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, de::Visitor, ser::SerializeSeq};

use crate::config::config_tag::ConfigTag;

type Matrix = DMatrix<f32>;

// Wrapper struct so that the nalgebra crate can be extended
#[derive(Clone, Debug, ConfigTag, PartialEq)]
pub struct MxNMatrix(pub Matrix);
impl MxNMatrix {
    pub fn rand(rows: usize, cols: usize) -> MxNMatrix {
        let mut rng = rand::rng();
        let mut m = DMatrix::zeros(rows, cols);

        for cell in m.iter_mut() {
            *cell = rng.random_range(-1.0..=1.0);
        }

        return MxNMatrix(m);
    }

    pub fn insert_row(&mut self, i: usize) {
        let temp = self.0.clone();
        self.0 = temp.insert_row(i, 0.0);
    }

    pub fn remove_row(&mut self, i: usize) {
        let temp = self.0.clone();
        self.0 = temp.remove_row(i);
    }

    pub fn insert_col(&mut self, i: usize) {
        let temp = self.0.clone();
        self.0 = temp.insert_column(i, 0.0);
    }

    pub fn remove_col(&mut self, i: usize) {
        let temp = self.0.clone();
        self.0 = temp.remove_column(i);
    }
}
impl Serialize for MxNMatrix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let m = &self.0;
        // Allocate space for sequence
        let mut m_seq = serializer.serialize_seq(Some(1 + m.iter().len()))?;

        // Add matrix shape data to sequence
        m_seq.serialize_element(&m.shape().0)?;
        m_seq.serialize_element(&m.shape().1)?;

        // Add each cell value to the sequence
        for cell in m.iter() {
            m_seq.serialize_element(&cell)?;
        }
        return m_seq.end();
    }
}
impl<'de> Deserialize<'de> for MxNMatrix {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Visitor for deserializing MxMMatrix
        struct MxMMatrixVisitor;
        impl<'de> Visitor<'de> for MxMMatrixVisitor {
            type Value = MxNMatrix;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                return formatter.write_str("Matrix");
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                // Get the shape of the matrix
                let shape = [
                    seq.next_element::<usize>()?.unwrap(),
                    seq.next_element::<usize>()?.unwrap(),
                ];

                // Loop through element to get matrix data
                let mut data = vec![];
                loop {
                    let cell = seq.next_element::<f32>()?;
                    match cell {
                        Some(val) => data.push(val),
                        None => break,
                    }
                }

                return Ok(MxNMatrix(Matrix::from_vec(shape[0], shape[1], data)));
            }
        }

        let m = d.deserialize_seq(MxMMatrixVisitor)?;
        return Ok(m);
    }
}

#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use crate::{
        config::config_tag::Config,
        world::matrix::{self, MxNMatrix},
    };

    #[test]
    fn matrix_save_load() {
        let matrix = MxNMatrix::rand(0, 0);
        let path = Path::new("../tmp/matrix.json");
        matrix.save_cfg(&path);
        assert_eq!(matrix, MxNMatrix::load_cfg(path));
        fs::remove_file(path).unwrap();
    }
}
