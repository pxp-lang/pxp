use crate::{ArithmeticOperationKind, AssignmentOperationKind, BitwiseOperationKind, ComparisonOperationKind, Expression, LogicalOperationKind};
use pxp_span::Span;

impl AssignmentOperationKind {
    pub fn left(&self) -> &Expression {
        match self {
            AssignmentOperationKind::Assign { left, .. } => left.as_ref(),
            AssignmentOperationKind::Addition { left, .. } => left.as_ref(),
            AssignmentOperationKind::Subtraction { left, .. } => left.as_ref(),
            AssignmentOperationKind::Multiplication { left, .. } => left.as_ref(),
            AssignmentOperationKind::Division { left, .. } => left.as_ref(),
            AssignmentOperationKind::Modulo { left, .. } => left.as_ref(),
            AssignmentOperationKind::Exponentiation { left, .. } => left.as_ref(),
            AssignmentOperationKind::Concat { left, .. } => left.as_ref(),
            AssignmentOperationKind::BitwiseAnd { left, .. } => left.as_ref(),
            AssignmentOperationKind::BitwiseOr { left, .. } => left.as_ref(),
            AssignmentOperationKind::BitwiseXor { left, .. } => left.as_ref(),
            AssignmentOperationKind::LeftShift { left, .. } => left.as_ref(),
            AssignmentOperationKind::RightShift { left, .. } => left.as_ref(),
            AssignmentOperationKind::Coalesce { left, .. } => left.as_ref(),
        }
    }

    pub fn right(&self) -> &Expression {
        match self {
            AssignmentOperationKind::Assign { right, .. } => right.as_ref(),
            AssignmentOperationKind::Addition { right, .. } => right.as_ref(),
            AssignmentOperationKind::Subtraction { right, .. } => right.as_ref(),
            AssignmentOperationKind::Multiplication { right, .. } => right.as_ref(),
            AssignmentOperationKind::Division { right, .. } => right.as_ref(),
            AssignmentOperationKind::Modulo { right, .. } => right.as_ref(),
            AssignmentOperationKind::Exponentiation { right, .. } => right.as_ref(),
            AssignmentOperationKind::Concat { right, .. } => right.as_ref(),
            AssignmentOperationKind::BitwiseAnd { right, .. } => right.as_ref(),
            AssignmentOperationKind::BitwiseOr { right, .. } => right.as_ref(),
            AssignmentOperationKind::BitwiseXor { right, .. } => right.as_ref(),
            AssignmentOperationKind::LeftShift { right, .. } => right.as_ref(),
            AssignmentOperationKind::RightShift { right, .. } => right.as_ref(),
            AssignmentOperationKind::Coalesce { right, .. } => right.as_ref(),
        }
    }

    pub fn operator(&self) -> &Span {
        match self {
            AssignmentOperationKind::Assign { equals, .. } => equals,
            AssignmentOperationKind::Addition { plus_equals, .. } => plus_equals,
            AssignmentOperationKind::Subtraction { minus_equals, .. } => minus_equals,
            AssignmentOperationKind::Multiplication {
                asterisk_equals, ..
            } => asterisk_equals,
            AssignmentOperationKind::Division { slash_equals, .. } => slash_equals,
            AssignmentOperationKind::Modulo { percent_equals, .. } => percent_equals,
            AssignmentOperationKind::Exponentiation { pow_equals, .. } => pow_equals,
            AssignmentOperationKind::Concat { dot_equals, .. } => dot_equals,
            AssignmentOperationKind::BitwiseAnd {
                ampersand_equals, ..
            } => ampersand_equals,
            AssignmentOperationKind::BitwiseOr { pipe_equals, .. } => pipe_equals,
            AssignmentOperationKind::BitwiseXor { caret_equals, .. } => caret_equals,
            AssignmentOperationKind::LeftShift {
                left_shift_equals, ..
            } => left_shift_equals,
            AssignmentOperationKind::RightShift {
                right_shift_equals, ..
            } => right_shift_equals,
            AssignmentOperationKind::Coalesce {
                coalesce_equals, ..
            } => coalesce_equals,
        }
    }
}

impl ComparisonOperationKind {
    pub fn left(&self) -> &Expression {
        match self {
            ComparisonOperationKind::Equal { left, .. } => left,
            ComparisonOperationKind::Identical { left, .. } => left,
            ComparisonOperationKind::NotEqual { left, .. } => left,
            ComparisonOperationKind::AngledNotEqual { left, .. } => left,
            ComparisonOperationKind::NotIdentical { left, .. } => left,
            ComparisonOperationKind::LessThan { left, .. } => left,
            ComparisonOperationKind::GreaterThan { left, .. } => left,
            ComparisonOperationKind::LessThanOrEqual { left, .. } => left,
            ComparisonOperationKind::GreaterThanOrEqual { left, .. } => left,
            ComparisonOperationKind::Spaceship { left, .. } => left,
        }
    }

    pub fn right(&self) -> &Expression {
        match self {
            ComparisonOperationKind::Equal { right, .. } => right,
            ComparisonOperationKind::Identical { right, .. } => right,
            ComparisonOperationKind::NotEqual { right, .. } => right,
            ComparisonOperationKind::AngledNotEqual { right, .. } => right,
            ComparisonOperationKind::NotIdentical { right, .. } => right,
            ComparisonOperationKind::LessThan { right, .. } => right,
            ComparisonOperationKind::GreaterThan { right, .. } => right,
            ComparisonOperationKind::LessThanOrEqual { right, .. } => right,
            ComparisonOperationKind::GreaterThanOrEqual { right, .. } => right,
            ComparisonOperationKind::Spaceship { right, .. } => right,
        }
    }

    pub fn set_right(&mut self, right: Box<Expression>) {
        match self {
            ComparisonOperationKind::Equal { right: r, .. } => *r = right,
            ComparisonOperationKind::Identical { right: r, .. } => *r = right,
            ComparisonOperationKind::NotEqual { right: r, .. } => *r = right,
            ComparisonOperationKind::AngledNotEqual { right: r, .. } => *r = right,
            ComparisonOperationKind::NotIdentical { right: r, .. } => *r = right,
            ComparisonOperationKind::LessThan { right: r, .. } => *r = right,
            ComparisonOperationKind::GreaterThan { right: r, .. } => *r = right,
            ComparisonOperationKind::LessThanOrEqual { right: r, .. } => *r = right,
            ComparisonOperationKind::GreaterThanOrEqual { right: r, .. } => *r = right,
            ComparisonOperationKind::Spaceship { right: r, .. } => *r = right,
        }
    }
}

impl BitwiseOperationKind {
    pub fn left(&self) -> Option<&Expression> {
        match self {
            BitwiseOperationKind::And { left, .. } => Some(left),
            BitwiseOperationKind::Or { left, .. } => Some(left),
            BitwiseOperationKind::Xor { left, .. } => Some(left),
            BitwiseOperationKind::LeftShift { left, .. } => Some(left),
            BitwiseOperationKind::RightShift { left, .. } => Some(left),
            BitwiseOperationKind::Not { .. } => None,
        }
    }

    pub fn right(&self) -> &Expression {
        match self {
            BitwiseOperationKind::And { right, .. } => right,
            BitwiseOperationKind::Or { right, .. } => right,
            BitwiseOperationKind::Xor { right, .. } => right,
            BitwiseOperationKind::LeftShift { right, .. } => right,
            BitwiseOperationKind::RightShift { right, .. } => right,
            BitwiseOperationKind::Not { right, .. } => right,
        }
    }

    pub fn set_right(&mut self, right: Box<Expression>) {
        match self {
            BitwiseOperationKind::And { right: r, .. } => *r = right,
            BitwiseOperationKind::Or { right: r, .. } => *r = right,
            BitwiseOperationKind::Xor { right: r, .. } => *r = right,
            BitwiseOperationKind::LeftShift { right: r, .. } => *r = right,
            BitwiseOperationKind::RightShift { right: r, .. } => *r = right,
            BitwiseOperationKind::Not { right: r, .. } => *r = right,
        }
    }
}

impl ArithmeticOperationKind {
    pub fn left(&self) -> Option<&Expression> {
        match self {
            ArithmeticOperationKind::Addition { id, left, plus, right } => Some(left),
            ArithmeticOperationKind::Subtraction { id, left, minus, right } => Some(left),
            ArithmeticOperationKind::Multiplication { id, left, asterisk, right } => Some(left),
            ArithmeticOperationKind::Division { id, left, slash, right } => Some(left),
            ArithmeticOperationKind::Modulo { id, left, percent, right } => Some(left),
            ArithmeticOperationKind::Exponentiation { id, left, pow, right } => Some(left),
            ArithmeticOperationKind::Negative { id, minus, right } => None,
            ArithmeticOperationKind::Positive { id, plus, right } => None,
            ArithmeticOperationKind::PreIncrement { id, increment, right } => None,
            ArithmeticOperationKind::PostIncrement { id, left, increment } => Some(left),
            ArithmeticOperationKind::PreDecrement { id, decrement, right } => None,
            ArithmeticOperationKind::PostDecrement { id, left, decrement } => Some(left),
        }
    }

    pub fn right(&self) -> Option<&Expression> {
        match self {
            ArithmeticOperationKind::Addition { id, left, plus, right } => Some(right),
            ArithmeticOperationKind::Subtraction { id, left, minus, right } => Some(right),
            ArithmeticOperationKind::Multiplication { id, left, asterisk, right } => Some(right),
            ArithmeticOperationKind::Division { id, left, slash, right } => Some(right),
            ArithmeticOperationKind::Modulo { id, left, percent, right } => Some(right),
            ArithmeticOperationKind::Exponentiation { id, left, pow, right } => Some(right),
            ArithmeticOperationKind::Negative { id, minus, right } => Some(right),
            ArithmeticOperationKind::Positive { id, plus, right } => Some(right),
            ArithmeticOperationKind::PreIncrement { id, increment, right } => Some(right),
            ArithmeticOperationKind::PostIncrement { id, left, increment } => None,
            ArithmeticOperationKind::PreDecrement { id, decrement, right } => Some(right),
            ArithmeticOperationKind::PostDecrement { id, left, decrement } => None,
        }
    }

    pub fn set_right(&mut self, right: Box<Expression>) {
        match self {
            ArithmeticOperationKind::Addition { right: r, .. } => *r = right,
            ArithmeticOperationKind::Subtraction { right: r, .. } => *r = right,
            ArithmeticOperationKind::Multiplication { right: r, .. } => *r = right,
            ArithmeticOperationKind::Division { right: r, .. } => *r = right,
            ArithmeticOperationKind::Modulo { right: r, .. } => *r = right,
            ArithmeticOperationKind::Exponentiation { right: r, .. } => *r = right,
            ArithmeticOperationKind::Negative { right: r, .. } => *r = right,
            ArithmeticOperationKind::Positive { right: r, .. } => *r = right,
            ArithmeticOperationKind::PreIncrement { right: r, .. } => *r = right,
            ArithmeticOperationKind::PreDecrement { right: r, .. } => *r = right,
            _ => {}
        }
    }
}

impl LogicalOperationKind {
    pub fn left(&self) -> Option<&Expression> {
        match self {
            LogicalOperationKind::And { id, left, double_ampersand, right } => Some(left),
            LogicalOperationKind::Or { id, left, double_pipe, right } => Some(left),
            LogicalOperationKind::Not { id, bang, right } => None,
            LogicalOperationKind::LogicalAnd { id, left, and, right } => Some(left),
            LogicalOperationKind::LogicalOr { id, left, or, right } => Some(left),
            LogicalOperationKind::LogicalXor { id, left, xor, right } => Some(left),
        }
    }

    pub fn right(&self) -> &Expression {
        match self {
            LogicalOperationKind::And { id, left, double_ampersand, right } => right,
            LogicalOperationKind::Or { id, left, double_pipe, right } => right,
            LogicalOperationKind::Not { id, bang, right } => right,
            LogicalOperationKind::LogicalAnd { id, left, and, right } => right,
            LogicalOperationKind::LogicalOr { id, left, or, right } => right,
            LogicalOperationKind::LogicalXor { id, left, xor, right } => right,
        }
    }

    pub fn set_right(&mut self, right: Box<Expression>) {
        match self {
            LogicalOperationKind::And { right: r, .. } => *r = right,
            LogicalOperationKind::Or { right: r, .. } => *r = right,
            LogicalOperationKind::Not { right: r, .. } => *r = right,
            LogicalOperationKind::LogicalAnd { right: r, .. } => *r = right,
            LogicalOperationKind::LogicalOr { right: r, .. } => *r = right,
            LogicalOperationKind::LogicalXor { right: r, .. } => *r = right,
        }
    }
}