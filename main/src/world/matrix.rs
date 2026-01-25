use nalgebra::DMatrix;
use serde::{Deserialize, Deserializer, Serialize, de::Visitor, ser::SerializeSeq};

pub type Matrix = DMatrix<f32>;

// Wrapper struct so that the nalgebra crate can be extended
#[derive(Clone, Debug)]
pub struct MxNMatrix(pub Matrix);
impl Serialize for MxNMatrix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let m = &self.0;
        // Allocate space for sequence
        let mut m_seq = serializer.serialize_seq(Some(m.iter().len()))?;

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
