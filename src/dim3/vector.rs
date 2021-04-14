use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub};
use {IUnit, Unit, UNIT_PI};

/// Represents a 3D vector.
///
/// Example usage:
///
/// ```
/// use rscsg::dim3::Vector;
/// Vector(1f32, 2f32, 3f32);
/// ```

#[derive(Clone, Copy, Debug)]
pub struct Vector(pub Unit, pub Unit, pub Unit);

#[derive(Clone, Copy, Eq, Debug)]
pub struct IVector(pub i32, pub i32, pub i32);

impl Vector {
    #[inline]
    pub fn x(&self) -> Unit {
        self.0
    }

    #[inline]
    pub fn y(&self) -> Unit {
        self.1
    }

    #[inline]
    pub fn z(&self) -> Unit {
        self.2
    }

    pub fn negate(&self) -> Self {
        Vector(-self.0, -self.1, -self.2)
    }

    pub fn dot(&self, other: Vector) -> Unit {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    /// Lerp. Linear interpolation from `self` to `other`
    pub fn lerp(&self, other: Vector, t: Unit) -> Self {
        let me = *self;
        me + (other - me) * t
    }

    pub fn length(&self) -> Unit {
        self.dot(*self).sqrt()
    }

    /// Normalize length of vector to 1.
    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    /// Cross product with another vector.
    pub fn cross(&self, other: Vector) -> Self {
        Vector(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn rotate(&self, axis: Vector, angle_deg: Unit) -> Self {
        let va: Unit = self.dot(axis);
        let vprep = *self - axis * va;
        let vprep_len = vprep.length();

        if vprep_len == 0. {
            *self
        } else {
            let cos_angle = (UNIT_PI * angle_deg / 180.).cos();
            let sin_angle = (UNIT_PI * angle_deg / 180.).sin();

            let u0 = vprep.normalize();
            let u1 = u0.cross(axis);
            let vcos = vprep_len * cos_angle;
            let vsin = vprep_len * sin_angle;
            axis * va + u0 * vcos + u1 * vsin
        }
    }

    pub fn discreet(&self, step: Unit) -> IVector {
        fn conv(x: Unit, d: Unit) -> IUnit {
            (x * d).round() as i32
        }

        IVector(conv(self.0, step), conv(self.1, step), conv(self.2, step))
    }

    /// Make a new vector which is orthogonal to `self`.
    pub fn make_orthogonal(&self) -> Self {
        *self
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        self.negate()
    }
}

impl Mul<Unit> for Vector {
    type Output = Self;

    fn mul(self, rhs: Unit) -> Self {
        Vector(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<Unit> for Vector {
    type Output = Self;

    fn div(self, rhs: Unit) -> Self {
        let inv = 1. / rhs;
        Vector(self.0 * inv, self.1 * inv, self.2 * inv)
    }
}

impl Hash for IVector {
    fn hash<H: Hasher>(&self, hashsum: &mut H) {
        self.0.hash(hashsum);
        self.1.hash(hashsum);
        self.2.hash(hashsum);
    }
}

impl PartialEq for IVector {
    fn eq(&self, rhs: &IVector) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1 && self.2 == rhs.2
    }
}
