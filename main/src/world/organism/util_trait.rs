use crate::world::organism::{body::Body, brain::Brain, organism::Organism};

pub trait OrganismAccessor {
    fn get_mut_organism<'a>(&'a mut self) -> &'a mut Organism;
    fn get_mut_brain<'a>(&'a mut self) -> Option<&'a mut Brain>;
    fn get_mut_body<'a>(&'a mut self) -> &'a mut Body;

    fn get_organism<'a>(&'a self) -> &'a Organism;
    fn get_brain<'a>(&'a self) -> &'a Option<Brain>;
    fn get_body<'a>(&'a self) -> &'a Body;
}
