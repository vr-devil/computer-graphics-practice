use std::ops::{Add, Div, Mul, Neg, Sub};


#[derive(PartialOrd, PartialEq)]
#[derive(Debug)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Div<f32> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Vector3 {

    pub fn normalize(&self) -> Self {
        self / self.norm()
    }

    pub fn norm(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
}

impl Mul<&Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Neg for &Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<&Vector3> for &Vector3 {
    type Output = f32;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        self.dot(rhs)
    }
}

mod test_vector3 {
    use crate::math::Vector3;

    #[test]
    fn test_add() {
        let a = Vector3 { x: 1., y: 2., z: 3. };
        let b = Vector3 { x: 1., y: 2., z: 3. };

        assert_eq!(Vector3 { x: 2., y: 4., z: 6. }, &a + &b);
        assert_eq!(Vector3 { x: 2., y: 4., z: 6. }, &b + &a);
        assert_eq!(Vector3 { x: 2., y: 4., z: 6. }, &b + &a);
    }

    #[test]
    fn test_sub() {
        let a = Vector3 { x: 1., y: 2., z: 3. };
        let b = Vector3 { x: 3., y: 2., z: 1. };
        let b = Vector3 { x: 3., y: 2., z: 1. };

        assert_eq!(Vector3 { x: -2., y: 0., z: 2. }, &a - &b);
        assert_eq!(Vector3 { x: -2., y: 0., z: 2. }, &a - &b);
        assert_eq!(Vector3 { x: 2., y: 0., z: -2. }, &b - &a);
        assert_eq!(Vector3 { x: 2., y: 0., z: -2. }, &b - &a);
    }

    #[test]
    fn test_dot() {
        let a = Vector3 {x: 1., y: 2., z: 3.};
        let b = Vector3 {x: 1., y: 2., z: 3.};

        assert_eq!(14., a.dot(&b));
        assert_eq!(14., b.dot(&a));
        assert_eq!(14., &a * &b);
        assert_eq!(14., &b * &a);
    }

    #[test]
    fn test_cross() {
        let x = super::X;
        let y = super::Y;
        let z = super::Z;

        assert_eq!(x, y.cross(&z));
        assert_eq!(y, z.cross(&x));
        assert_eq!(z, x.cross(&y));

        assert_eq!(-&x, z.cross(&y));
        assert_eq!(-&y, x.cross(&z));
        assert_eq!(-&z, y.cross(&x));
    }
}

const X: Vector3 = Vector3 { x: 1., y: 0., z: 0. };
const Y: Vector3 = Vector3 { x: 0., y: 1., z: 0. };
const Z: Vector3 = Vector3 { x: 0., y: 0., z: 1. };

struct Basis {
    u: Vector3,
    v: Vector3,
    w: Vector3
}

impl Basis {
    pub fn from_single_vector(a: &Vector3) -> Self {
        let w = a.normalize();

        let t = {
            let (mut x, mut y, mut z) = (w.x.abs(), w.y.abs(), w.z.abs());
            if x > y {
                if y > z {
                    z = 1.;
                } else {
                    y = 1.
                }
            } else {
                if x > z {
                    z = 1.
                } else {
                    x = 1.
                }
            }

            Vector3{x, y, z}
        };

        let u = {
            t.cross(&w).normalize()
        };

        let v = w.cross(&u);

        Self { u, v, w }
    }
}

mod test_basis {
    use crate::math::{Basis, Vector3};

    #[test]
    fn test_from_single_vector() {

        let basis = Basis::from_single_vector(&Vector3 { x: 2.5, y: 1.5, z: 0.5 });

        let (u, v, w) = (basis.u, basis.v, basis.w);

        assert_eq!(u.norm().round(), 1.);
        assert_eq!(v.norm().round(), 1.);
        assert_eq!(w.norm().round(), 1.);
    }
}