pub trait Env {
    fn get(&self, x: isize, y: isize) -> f32;
    // fn set(&mut self, x: isize, y: isize, value: f32);
    fn delta(&mut self, x: isize, y: isize, delta: &mut f32);
    fn max(&self) -> f32;

    fn update(&mut self, dt: f32);
}
