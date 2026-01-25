use bevy::{
    log::{error, info},
    math::usize,
};
use nalgebra::DMatrix;
use rand::{Rng, rngs::ThreadRng, thread_rng};
use serde::{Deserialize, Deserializer, Serialize, de::Visitor, ser::SerializeSeq};

use crate::world::{
    matrix::MxNMatrix,
    organism::mutation::{Brain as BrainMutation, Mutable, Mutation},
};

// use super::mutation::{BrainMutateType, Mutable, Mutation};

// Basic neural network
#[derive(Clone, Serialize, Deserialize)]
pub struct Brain {
    pub weights: Vec<MxNMatrix>,
    pub biases: Vec<MxNMatrix>,
}
impl Default for Brain {
    fn default() -> Self {
        return Self::new(vec![2, 4, 4, 0]);
    }
}
impl Mutable for Brain {
    fn mutate(&mut self, mutation: Mutation) -> bool {
        match mutation {
            Mutation::Brain(mutation) => {
                info!("Mutating brain: {mutation:?}");
                match mutation {
                    BrainMutation::AddInput { index } => self.add_input(index),
                    BrainMutation::RemoveInput { index } => self.remove_input(index),
                    BrainMutation::AddOutput { index } => self.add_output(index),
                    BrainMutation::RemoveOutput { index } => self.remove_output(index),
                    BrainMutation::Learn {
                        learn_rate: _,
                        learn_factor: _,
                    } => {
                        error!("Mutate structure called with learn mutation, this shouldn't happen")
                    }
                }
            }
            _ => unreachable!(
                "Tried to mutate brain using invalid mutation {:?}",
                mutation
            ),
        }

        return true;
    }
}
impl Brain {
    // Create a new brain based on the structure provided
    pub fn new(structure: Vec<usize>) -> Self {
        let mut weights = vec![];
        let mut biases = vec![];
        let num_layers = structure.len();

        for i in 1..num_layers {
            weights.push(gen_rand_matrix(structure[i - 1], structure[i]));
            biases.push(gen_rand_matrix(1, structure[i]));
        }

        return Self { weights, biases };
    }

    pub fn get_num_inputs(&self) -> usize {
        return self.weights[0].0.shape().0;
    }
    pub fn get_num_outputs(&self) -> usize {
        return self.weights[self.weights.len() - 1].0.shape().0;
    }

    pub fn is_valid(&self) -> bool {
        let mut valid = true;
        for len in self.get_structure() {
            if len == 0 {
                valid = false;
            }
        }
        return valid;
    }

    // pub fn mutate(&mut self, mutation: BrainMutation) {
    //     match mutation {
    //         BrainMutateType::AddInput(Some(i)) => {
    //             for _ in 0..i {
    //                 self.add_input();
    //             }
    //         }
    //         BrainMutateType::RemoveInput(Some(i)) => {
    //             for _ in 0..i {
    //                 self.remove_input();
    //             }
    //         }
    //         BrainMutateType::AddOutput(Some(i)) => {
    //             for _ in 0..i {
    //                 self.add_output();
    //             }
    //         }
    //         BrainMutateType::RemoveOutput(Some(i)) => {
    //             for _ in 0..i {
    //                 self.remove_output();
    //             }
    //         }
    //         BrainMutateType::AddHidden(Some(i)) => {
    //             self.add_hidden(i);
    //         }
    //         BrainMutateType::RemoveHidden(Some(i)) => {
    //             self.remove_hidden(i);
    //         }
    //         BrainMutateType::AddLayer(Some(i)) => {
    //             self.add_layer(i);
    //         }
    //         BrainMutateType::RemoveLayer(Some(i)) => {
    //             self.remove_layer(i);
    //         }
    //         _ => unreachable!("Brain mutation attempted without value {:?}", mutation),
    //     };
    // }

    fn add_input(&mut self, i: usize) {
        insert_row(&mut self.weights[0], i);
    }

    fn remove_input(&mut self, i: usize) {
        remove_row(&mut self.weights[0], i);
    }

    fn add_output(&mut self, i: usize) {
        let num_weights = self.weights.len();
        let num_biases = self.biases.len();

        insert_col(&mut self.weights[num_weights - 1], i);
        insert_col(&mut self.biases[num_biases - 1], i);
    }

    fn remove_output(&mut self, i: usize) {
        let num_weights = self.weights.len();
        let num_biases = self.biases.len();

        remove_col(&mut self.weights[num_weights - 1], i);
        remove_col(&mut self.biases[num_biases - 1], i);
    }

    // fn add_hidden(&mut self, i: usize) {
    //     insert_col(&mut self.weights[i - 1]);
    //     insert_row(&mut self.weights[i]);
    //     insert_col(&mut self.biases[i - 1]);
    // }

    // fn remove_hidden(&mut self, i: usize) {
    //     remove_col(&mut self.weights[i - 1]);
    //     remove_row(&mut self.weights[i]);
    //     remove_col(&mut self.biases[i - 1]);
    // }

    fn add_layer(&mut self, i: usize) {
        let num_in = self.weights[i - 1].0.shape().1;
        let num_out = self.weights[i].0.shape().0;

        self.weights.insert(i, gen_rand_matrix(num_in, num_out));
        self.biases.insert(i, gen_rand_matrix(1, num_out));
    }

    fn remove_layer(&mut self, i: usize) {
        self.weights.remove(i);
        self.biases.remove(i - 1);
    }

    pub fn process(&self, input: Vec<f32>) -> Vec<f32> {
        let in_len = input.len();
        let len = self.weights[0].0.shape().0;
        if in_len != len {
            error!("brain received {}/{} inputs", in_len, len);
            return vec![0.0; self.get_num_outputs()];
        }

        // Feed forward input
        let x = DMatrix::from_vec(1, in_len, input);
        let mut y = x;
        for i in 0..self.weights.len() {
            y = y * &self.weights[i].0 + &self.biases[i].0;
            for cell in y.iter_mut() {
                *cell = cell.tanh();
            }
        }
        let output = y.iter().map(|x| *x).collect::<Vec<f32>>();

        return output;
    }

    // Mutate brain connections based on learning rate and learning factor
    fn mutate_connections(
        &mut self,
        rng: &mut ThreadRng,
        learning_rate: f32,
        learning_factor: f32,
    ) {
        for weight in self.weights.iter_mut() {
            Self::mutate_matrix(rng, weight, learning_rate, learning_factor);
        }

        for bias in self.biases.iter_mut() {
            Self::mutate_matrix(rng, bias, learning_rate, learning_factor);
        }
    }

    fn mutate_matrix(rng: &mut ThreadRng, m: &mut MxNMatrix, mut_rate: f32, mut_factor: f32) {
        for cell in m.0.iter_mut() {
            if rng.r#gen::<f32>() <= mut_rate {
                *cell += (rng.r#gen::<f32>() - 0.5) * mut_factor;
            }
        }
    }

    pub fn get_structure(&self) -> Vec<usize> {
        let mut structure = vec![self.weights[0].0.shape().0];

        let weights_structure = self
            .weights
            .iter()
            .map(|x| x.0.shape().1)
            .collect::<Vec<usize>>();

        let foo = self
            .biases
            .iter()
            .map(|x| x.0.shape().1)
            .collect::<Vec<usize>>();

        assert_eq!(*weights_structure, *foo);

        structure.extend(weights_structure);

        return structure;
    }
}

fn gen_rand_matrix(rows: usize, cols: usize) -> MxNMatrix {
    let mut rng = rand::thread_rng();
    let mut m = DMatrix::zeros(rows, cols);

    for cell in m.iter_mut() {
        *cell = rng.r#gen_range(-1.0..=1.0);
    }

    return MxNMatrix(m);
}

fn insert_row(m: &mut MxNMatrix, i: usize) {
    let temp = m.0.clone();
    // let rows = temp.shape().0;
    m.0 = temp.insert_row(i, 0.0);
}
fn remove_row(m: &mut MxNMatrix, i: usize) {
    let temp = m.0.clone();
    // let rows = temp.shape().0;
    m.0 = temp.remove_row(i);
}

fn insert_col(m: &mut MxNMatrix, i: usize) {
    let temp = m.0.clone();
    // let cols = temp.shape().1;
    m.0 = temp.insert_column(i, 0.0);
}
fn remove_col(m: &mut MxNMatrix, i: usize) {
    let temp = m.0.clone();
    // let cols = temp.shape().1;
    m.0 = temp.remove_column(i);
}
