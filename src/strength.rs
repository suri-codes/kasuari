//! Contains useful constants and functions for producing strengths for use in the constraint
//! solver. Each constraint added to the solver has an associated strength specifying the precedence
//! the solver should impose when choosing which constraints to enforce. It will try to enforce all
//! constraints, but if that is impossible the lowest strength constraints are the first to be
//! violated.
//!
//! Strengths are simply real numbers. The strongest legal strength is 1,001,001,000.0. The weakest
//! is 0.0. For convenience constants are declared for commonly used strengths. These are
//! `REQUIRED`, `STRONG`, `MEDIUM` and `WEAK`. Feel free to multiply these by other values to get
//! intermediate strengths. Note that the solver will clip given strengths to the legal range.
//!
//! `REQUIRED` signifies a constraint that cannot be violated under any circumstance. Use this
//! special strength sparingly, as the solver will fail completely if it find that not all of the
//! `REQUIRED` constraints can be satisfied. The other strengths represent fallible constraints.
//! These should be the most commonly used strenghts for use cases where violating a constraint is
//! acceptable or even desired.
//!
//! The solver will try to get as close to satisfying the constraints it violates as possible,
//! strongest first. This behaviour can be used (for example) to provide a "default" value for a
//! variable should no other stronger constraints be put upon it.

/// Create a constraint as a linear combination of STRONG, MEDIUM and WEAK strengths,
/// corresponding to `a` `b` and `c` respectively. The result is further multiplied by `w`.
pub fn create(a: f64, b: f64, c: f64, w: f64) -> f64 {
    (a * w).clamp(0.0, 1000.0) * 1_000_000.0
        + (b * w).clamp(0.0, 1000.0) * 1000.0
        + (c * w).clamp(0.0, 1000.0)
}
pub const REQUIRED: f64 = 1_001_001_000.0;
pub const STRONG: f64 = 1_000_000.0;
pub const MEDIUM: f64 = 1_000.0;
pub const WEAK: f64 = 1.0;

/// Clips a strength value to the legal range
pub fn clip(s: f64) -> f64 {
    s.clamp(0.0, REQUIRED)
}
