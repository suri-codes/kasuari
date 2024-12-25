//! Contains useful constants and functions for producing strengths for use in the constraint
//! solver. Each constraint added to the solver has an associated strength specifying the precedence
//! the solver should impose when choosing which constraints to enforce. It will try to enforce all
//! constraints, but if that is impossible the lowest strength constraints are the first to be
//! violated.
//!
//! Strengths are simply real numbers. The strongest legal strength is 1,001,001,000.0. The weakest
//! is 0.0. For convenience constants are declared for commonly used strengths. These are
//! [`REQUIRED`], [`STRONG`], [`MEDIUM`] and [`WEAK`]. Feel free to multiply these by other values
//! to get intermediate strengths. Note that the solver will clip given strengths to the legal
//! range.
//!
//! [`REQUIRED`] signifies a constraint that cannot be violated under any circumstance. Use this
//! special strength sparingly, as the solver will fail completely if it find that not all of the
//! [`REQUIRED`] constraints can be satisfied. The other strengths represent fallible constraints.
//! These should be the most commonly used strenghts for use cases where violating a constraint is
//! acceptable or even desired.
//!
//! The solver will try to get as close to satisfying the constraints it violates as possible,
//! strongest first. This behaviour can be used (for example) to provide a "default" value for a
//! variable should no other stronger constraints be put upon it.

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Strength(f64);

/// The required strength for a constraint. This is the strongest possible strength.
pub const REQUIRED: Strength = Strength(1_001_001_000.0);

/// A strong strength for a constraint. This is weaker than `REQUIRED` but stronger than `MEDIUM`.
pub const STRONG: Strength = Strength(1_000_000.0);

/// A medium strength for a constraint. This is weaker than `STRONG` but stronger than `WEAK`.
pub const MEDIUM: Strength = Strength(1_000.0);

/// A weak strength for a constraint. This is weaker than `MEDIUM` but stronger than `0.0`.
pub const WEAK: Strength = Strength(1.0);

impl Strength {
    /// Create a new strength with the given value, clipped to the legal range (0.0, REQUIRED)
    #[inline]
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, REQUIRED.value()))
    }

    /// Create a constraint as a linear combination of STRONG, MEDIUM and WEAK strengths,
    /// corresponding to `a` `b` and `c` respectively. The result is further multiplied by `w`.
    /// The result is clipped to the legal range.
    #[inline]
    pub fn create(strong: f64, medium: f64, weak: f64, multiplier: f64) -> Self {
        Self(
            (strong * multiplier).clamp(0.0, 1000.0) * 1_000_000.0
                + (medium * multiplier).clamp(0.0, 1000.0) * 1000.0
                + (weak * multiplier).clamp(0.0, 1000.0),
        )
    }

    #[inline]
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl std::ops::Mul<f64> for Strength {
    type Output = Strength;

    #[inline]
    fn mul(self, rhs: f64) -> Strength {
        Strength::new(self.0 * rhs)
    }
}

impl std::ops::Mul<Strength> for f64 {
    type Output = Strength;

    #[inline]
    fn mul(self, rhs: Strength) -> Strength {
        Strength::new(self * rhs.0)
    }
}

impl std::ops::Add<Strength> for Strength {
    type Output = Strength;

    #[inline]
    fn add(self, rhs: Strength) -> Strength {
        Strength::new(self.0 + rhs.0)
    }
}

impl std::ops::Sub<Strength> for Strength {
    type Output = Strength;

    #[inline]
    fn sub(self, rhs: Strength) -> Strength {
        Strength::new(self.0 - rhs.0)
    }
}

impl std::ops::AddAssign<Strength> for Strength {
    #[inline]
    fn add_assign(&mut self, rhs: Strength) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign<Strength> for Strength {
    #[inline]
    fn sub_assign(&mut self, rhs: Strength) {
        *self = *self - rhs;
    }
}

impl std::cmp::Ord for Strength {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::cmp::Eq for Strength {}
