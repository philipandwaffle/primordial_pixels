use std::{
    iter::Sum,
    ops::{Add, AddAssign, Neg},
};

#[derive(Debug)]
pub struct OutputConsumedInputProduced(pub [usize; 2]);
impl Sum for OutputConsumedInputProduced {
    fn sum<I: Iterator<Item = OutputConsumedInputProduced>>(iter: I) -> Self {
        let mut total = OutputConsumedInputProduced([0; 2]);
        for out_in in iter {
            total += out_in;
        }
        total
    }
}
impl AddAssign for OutputConsumedInputProduced {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
    }
}
impl Add for OutputConsumedInputProduced {
    type Output = OutputConsumedInputProduced;

    fn add(self, rhs: Self) -> Self::Output {
        OutputConsumedInputProduced([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}
impl Into<[usize; 2]> for OutputConsumedInputProduced {
    fn into(self) -> [usize; 2] {
        self.0
    }
}
