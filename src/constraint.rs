use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

use crate::{Expression, RelationalOperator};

#[derive(Debug)]
pub(crate) struct ConstraintData {
    pub(crate) expression: Expression,
    pub(crate) strength: f64,
    pub(crate) op: RelationalOperator,
}

/// A constraint, consisting of an equation governed by an expression and a relational operator,
/// and an associated strength.
#[derive(Clone, Debug)]
pub struct Constraint(Arc<ConstraintData>);

impl Constraint {
    /// Construct a new constraint from an expression, a relational operator and a strength.
    /// This corresponds to the equation `e op 0.0`, e.g. `x + y >= 0.0`. For equations with a
    /// non-zero right hand side, subtract it from the equation to give a zero right hand side.
    pub fn new(e: Expression, op: RelationalOperator, strength: f64) -> Constraint {
        Constraint(Arc::new(ConstraintData {
            expression: e,
            op,
            strength,
        }))
    }
    /// The expression of the left hand side of the constraint equation.
    pub fn expr(&self) -> &Expression {
        &self.0.expression
    }
    /// The relational operator governing the constraint.
    pub fn op(&self) -> RelationalOperator {
        self.0.op
    }
    /// The strength of the constraint that the solver will use.
    pub fn strength(&self) -> f64 {
        self.0.strength
    }
}

impl Hash for Constraint {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        use std::ops::Deref;
        hasher.write_usize(self.0.deref() as *const _ as usize);
    }
}

impl PartialEq for Constraint {
    fn eq(&self, other: &Constraint) -> bool {
        use std::ops::Deref;
        std::ptr::eq(self.0.deref(), other.0.deref())
    }
}

impl Eq for Constraint {}
