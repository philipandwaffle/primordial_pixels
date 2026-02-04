#[derive(Hash, PartialEq, Eq)]
pub enum LayerKey {
    Energy,
    Pheromone(usize),
}
