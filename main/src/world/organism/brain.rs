use std::collections::VecDeque;

use bevy::{log::info, math::usize};
use nalgebra::DMatrix;
use rand::{Rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use crate::{
    consts::{BASE_BRAIN_STRUCTURE, BASE_INPUT, BASE_OUTPUT},
    world::{
        matrix::MxNMatrix as Matrix,
        organism::mutation::{
            brain::Brain as BrainMutation,
            mutation::{Mutable, Mutation},
        },
    },
};

// use super::mutation::{BrainMutateType, Mutable, Mutation};

// Basic neural network
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Brain {
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    memory: Vec<f32>,
    input: VecDeque<f32>,
    output: VecDeque<f32>,
}
impl Default for Brain {
    fn default() -> Self {
        return Self::new(BASE_BRAIN_STRUCTURE.to_vec());
    }
}
impl Mutable for Brain {
    fn mutate(&mut self, mutation: &Mutation) -> bool {
        match mutation {
            Mutation::Brain(mutation) => {
                // info!("Mutating brain: {mutation:?}, {:?}", self.get_structure());
                match mutation {
                    BrainMutation::AddInput { index } => self.add_input(*index),
                    BrainMutation::RemoveInput { index } => self.remove_input(*index),
                    BrainMutation::AddOutput { index } => self.add_output(*index),
                    BrainMutation::RemoveOutput { index } => self.remove_output(*index),
                    BrainMutation::Learn {
                        learn_rate,
                        learn_factor,
                    } => {
                        self.mutate_connections(&mut rand::rng(), *learn_rate, *learn_factor);
                    }
                }
                // info!("Mutated brain: {:?}", self.get_structure());
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
            weights.push(Matrix::rand(structure[i - 1], structure[i]));
            biases.push(Matrix::rand(1, structure[i]));
        }

        return Self {
            weights,
            biases,
            memory: vec![0.0; BASE_OUTPUT],
            input: VecDeque::from(vec![0.0; BASE_INPUT]),
            output: VecDeque::from(vec![0.0; BASE_OUTPUT]),
        };
    }

    pub fn get_output(&self) -> VecDeque<f32> {
        return self.output.clone();
    }
    pub fn set_output(&mut self, output: VecDeque<f32>) {
        self.output = output;
    }
    pub fn set_input(&mut self, input: VecDeque<f32>) {
        self.input = input;
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

    fn add_input(&mut self, i: usize) {
        self.weights[0].insert_row(i);
    }

    fn remove_input(&mut self, i: usize) {
        self.weights[0].remove_row(i);
    }

    fn add_output(&mut self, i: usize) {
        let num_weights = self.weights.len();
        let num_biases = self.biases.len();

        self.weights[num_weights - 1].insert_col(i);
        self.biases[num_biases - 1].insert_col(i);
    }

    fn remove_output(&mut self, i: usize) {
        let num_weights = self.weights.len();
        let num_biases = self.biases.len();

        self.weights[num_weights - 1].remove_col(i);
        self.biases[num_biases - 1].remove_col(i);
    }

    pub fn process(&self) -> VecDeque<f32> {
        let input = Vec::from((&self).input.clone());
        let input = [&self.memory[..], &input[..]].concat();

        let in_len = input.len();
        let len = self.weights[0].0.shape().0;
        if in_len != len {
            panic!("brain received {}/{} inputs", in_len, len);
        }

        // Feed forward input
        let x = DMatrix::<f32>::from_vec(1, in_len, input);
        let mut y = x;
        for i in 0..self.weights.len() {
            y = y * &self.weights[i].0 + &self.biases[i].0;
            for cell in y.iter_mut() {
                *cell = cell.tanh();
            }
        }

        return y.iter().map(|x| *x).collect::<VecDeque<f32>>();
    }

    // Mutate brain connections based on learning rate and learning factor
    fn mutate_connections(&mut self, rng: &mut ThreadRng, learn_rate: f32, learn_factor: f32) {
        for weight in self.weights.iter_mut() {
            Self::mutate_matrix(rng, weight, learn_rate, learn_factor);
        }

        for bias in self.biases.iter_mut() {
            Self::mutate_matrix(rng, bias, learn_rate, learn_factor);
        }
    }

    fn mutate_matrix(rng: &mut ThreadRng, m: &mut Matrix, mut_rate: f32, mut_factor: f32) {
        for cell in m.0.iter_mut() {
            if rng.random::<f32>() <= mut_rate {
                *cell += (rng.random::<f32>() - 0.5) * mut_factor;
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

mod test {
    use crate::world::organism::brain::Brain;

    fn get_brain() -> Brain {
        Brain::new(vec![4, 4, 4])
    }

    #[test]
    fn remove_input() {
        let mut b = get_brain();
        println!("{:?}", b.get_structure());
        b.remove_input(0);
        println!("{:?}", b.get_structure());
    }
}
