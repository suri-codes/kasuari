use std::{
    hash::{Hash, Hasher},
    ops,
    sync::Arc,
};

use crate::{Expression, RelationalOperator, Term, Variable, WeightedRelation};

#[derive(Debug)]
struct Inner {
    expression: Expression,
    strength: f64,
    operator: RelationalOperator,
}

/// A constraint, consisting of an equation governed by an expression and a relational operator,
/// and an associated strength.
#[derive(Clone, Debug)]
pub struct Constraint(Arc<Inner>);

impl Constraint {
    /// Construct a new constraint from an expression, a relational operator and a strength.
    /// This corresponds to the equation `e op 0.0`, e.g. `x + y >= 0.0`. For equations with a
    /// non-zero right hand side, subtract it from the equation to give a zero right hand side.
    pub fn new(expression: Expression, operator: RelationalOperator, strength: f64) -> Constraint {
        Constraint(Arc::new(Inner {
            expression,
            operator,
            strength,
        }))
    }
    /// The expression of the left hand side of the constraint equation.
    pub fn expr(&self) -> &Expression {
        &self.0.expression
    }
    /// The relational operator governing the constraint.
    pub fn op(&self) -> RelationalOperator {
        self.0.operator
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

/// This is an intermediate type used in the syntactic sugar for specifying constraints. You should
/// not use it directly.
pub struct PartialConstraint(Expression, WeightedRelation);

impl PartialConstraint {
    /// Construct a new partial constraint from an expression and a relational operator.
    pub const fn new(expression: Expression, operator: WeightedRelation) -> PartialConstraint {
        PartialConstraint(expression, operator)
    }
}

impl ops::BitOr<f64> for PartialConstraint {
    type Output = Constraint;
    fn bitor(self, rhs: f64) -> Constraint {
        let (op, s) = self.1.into();
        Constraint::new(self.0 - rhs, op, s)
    }
}
impl ops::BitOr<f32> for PartialConstraint {
    type Output = Constraint;
    fn bitor(self, rhs: f32) -> Constraint {
        self.bitor(rhs as f64)
    }
}
impl ops::BitOr<Variable> for PartialConstraint {
    type Output = Constraint;
    fn bitor(self, rhs: Variable) -> Constraint {
        let (op, s) = self.1.into();
        Constraint::new(self.0 - rhs, op, s)
    }
}
impl ops::BitOr<Term> for PartialConstraint {
    type Output = Constraint;
    fn bitor(self, rhs: Term) -> Constraint {
        let (op, s) = self.1.into();
        Constraint::new(self.0 - rhs, op, s)
    }
}
impl ops::BitOr<Expression> for PartialConstraint {
    type Output = Constraint;
    fn bitor(self, rhs: Expression) -> Constraint {
        let (op, s) = self.1.into();
        Constraint::new(self.0 - rhs, op, s)
    }
}
