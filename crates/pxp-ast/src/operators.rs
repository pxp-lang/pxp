use crate::{AssignmentOperationKind, Expression};
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
