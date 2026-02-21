use crate::world::environment::field::Field;

pub trait Env<const N: usize> {
    fn field(&self) -> &Field<f32, N>;
    fn field_mut(&mut self) -> &mut Field<f32, N>;

    fn get(&self, x: isize, y: isize) -> f32;
    fn delta(&mut self, x: isize, y: isize, delta: &mut f32) {
        let new_val = self.get(x, y) - *delta;

        if new_val <= 0.0 {
            *delta = -new_val;
            self.field_mut().set(x, y, 0.0);
        } else {
            *delta = 0.0;
            self.field_mut().set(x, y, new_val);
        }
    }
    fn max(&self) -> f32;

    fn update(&mut self, dt: f32);
}
