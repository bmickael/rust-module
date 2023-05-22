#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Vector {
    i: f64,
    j: f64,
}

impl Vector {
    fn new(i: f64, j: f64) -> Self {
        Self { i, j }
    }
    fn dump_data(&self) {
        dbg!(self);
    }
    #[allow(dead_code)]
    fn add_value(&mut self, v: f64) {
        self.i += v;
        self.j += v;
    }
}

use std::fmt;

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}

use std::ops::{Add, AddAssign};

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            i: self.i + other.i,
            j: self.j + other.j,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

fn main() {
    let v = Vector::new(1.0, 2.0);
    v.dump_data();
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn add() {
        assert_eq!(
            Vector::new(3.0, 7.0),
            Vector::new(1.0, 2.0) + Vector::new(2.0, 5.0)
        );
    }

    #[test]
    fn add_assign() {
        let mut v2 = Vector::new(1.0, 2.0);
        v2 += Vector::new(2.0, 5.0);
        assert_eq!(Vector::new(3.0, 7.0), v2);
    }

    #[test]
    fn display() {
        assert_eq!("(1, 2)", format!("{}", Vector::new(1.0, 2.0)));
    }

    #[test]
    fn add_value() {
        let mut v2 = Vector::new(0.0, 1.0);
        v2.add_value(1.0);
        assert_eq!(Vector::new(1.0, 2.0), v2);
    }
}
