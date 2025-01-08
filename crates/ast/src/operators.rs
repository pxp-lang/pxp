use crate::{
    ArithmeticOperationKind, BitwiseOperationKind,
    ComparisonOperationKind, Expression, LogicalOperationKind,
};

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
            ArithmeticOperationKind::Addition { left, .. } => Some(left),
            ArithmeticOperationKind::Subtraction { left, .. } => Some(left),
            ArithmeticOperationKind::Multiplication { left, .. } => Some(left),
            ArithmeticOperationKind::Division { left, .. } => Some(left),
            ArithmeticOperationKind::Modulo { left, .. } => Some(left),
            ArithmeticOperationKind::Exponentiation { left, .. } => Some(left),
            ArithmeticOperationKind::Negative { .. } => None,
            ArithmeticOperationKind::Positive { .. } => None,
            ArithmeticOperationKind::PreIncrement { .. } => None,
            ArithmeticOperationKind::PostIncrement { left, .. } => Some(left),
            ArithmeticOperationKind::PreDecrement { .. } => None,
            ArithmeticOperationKind::PostDecrement { left, .. } => Some(left),
        }
    }

    pub fn right(&self) -> Option<&Expression> {
        match self {
            ArithmeticOperationKind::Addition { right, .. } => Some(right),
            ArithmeticOperationKind::Subtraction { right, .. } => Some(right),
            ArithmeticOperationKind::Multiplication { right, .. } => Some(right),
            ArithmeticOperationKind::Division { right, .. } => Some(right),
            ArithmeticOperationKind::Modulo { right, .. } => Some(right),
            ArithmeticOperationKind::Exponentiation { right, .. } => Some(right),
            ArithmeticOperationKind::Negative { right, .. } => Some(right),
            ArithmeticOperationKind::Positive { right, .. } => Some(right),
            ArithmeticOperationKind::PreIncrement { right, .. } => Some(right),
            ArithmeticOperationKind::PostIncrement { .. } => None,
            ArithmeticOperationKind::PreDecrement { right, .. } => Some(right),
            ArithmeticOperationKind::PostDecrement { .. } => None,
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
            LogicalOperationKind::And { left, .. } => Some(left),
            LogicalOperationKind::Or { left, .. } => Some(left),
            LogicalOperationKind::Not { .. } => None,
            LogicalOperationKind::LogicalAnd { left, .. } => Some(left),
            LogicalOperationKind::LogicalOr { left, .. } => Some(left),
            LogicalOperationKind::LogicalXor { left, .. } => Some(left),
        }
    }

    pub fn right(&self) -> &Expression {
        match self {
            LogicalOperationKind::And { right, .. } => right,
            LogicalOperationKind::Or { right, .. } => right,
            LogicalOperationKind::Not { right, .. } => right,
            LogicalOperationKind::LogicalAnd { right, .. } => right,
            LogicalOperationKind::LogicalOr { right, .. } => right,
            LogicalOperationKind::LogicalXor { right, .. } => right,
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
