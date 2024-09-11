use std::fmt::{Display, Formatter};

mod backed_enum_type;
mod comments;
mod generated;
mod id;
mod node;
mod visibility;
pub mod visitor;

pub use generated::*;
pub use id::HasId;
pub use node::Node;
use pxp_span::{Span, Spanned};
use pxp_token::{Token, TokenKind};
pub use visibility::*;

pub mod data_type;
pub mod identifiers;
pub mod literals;
pub mod modifiers;
pub mod name;
pub mod operators;
pub mod properties;
mod spanned;
pub mod utils;
pub mod variables;

impl Display for UseKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UseKind::Normal => write!(f, "use"),
            UseKind::Function => write!(f, "use function"),
            UseKind::Const => write!(f, "use const"),
        }
    }
}

impl Statement {
    pub fn new(id: NodeId, kind: StatementKind, span: Span, comments: CommentGroup) -> Self {
        Self {
            id,
            span,
            kind,
            comments,
        }
    }
}

impl Expression {
    pub fn new(id: NodeId, kind: ExpressionKind, span: Span, comments: CommentGroup) -> Self {
        Self {
            id,
            span,
            kind,
            comments,
        }
    }

    pub fn missing(id: NodeId, span: Span) -> Self {
        Self::new(
            id,
            ExpressionKind::Missing(MissingExpression { id, span }),
            span,
            CommentGroup::default(),
        )
    }

    pub fn noop(id: NodeId, span: Span) -> Self {
        Self::new(
            id,
            ExpressionKind::Noop(span),
            span,
            CommentGroup::default(),
        )
    }
}

impl From<Token> for CastKind {
    fn from(token: Token) -> Self {
        match token.kind {
            TokenKind::StringCast | TokenKind::BinaryCast => Self::String(token.span),
            TokenKind::ObjectCast => Self::Object(token.span),
            TokenKind::BoolCast | TokenKind::BooleanCast => Self::Bool(token.span),
            TokenKind::IntCast | TokenKind::IntegerCast => Self::Int(token.span),
            TokenKind::FloatCast | TokenKind::DoubleCast | TokenKind::RealCast => {
                Self::Float(token.span)
            }
            TokenKind::UnsetCast => Self::Unset(token.span),
            TokenKind::ArrayCast => Self::Array(token.span),
            _ => unreachable!(),
        }
    }
}

impl From<&TokenKind> for CastKind {
    fn from(kind: &TokenKind) -> Self {
        kind.into()
    }
}

impl From<Token> for SpecialNameKind {
    fn from(token: Token) -> Self {
        match token.kind {
            TokenKind::Self_ => Self::Self_(token.span),
            TokenKind::Parent => Self::Parent(token.span),
            TokenKind::Static => Self::Static(token.span),
            _ => unreachable!(),
        }
    }
}

impl<'a> Node<'a> {
    pub fn new(id: NodeId, kind: NodeKind<'a>, span: Span) -> Self {
        Self { id, kind, span }
    }
}

impl Spanned for Argument {
    fn span(&self) -> Span {
        match self {
            Argument::Positional(arg) => arg.span(),
            Argument::Named(arg) => arg.span(),
        }
    }
}

impl Spanned for StringPart {
    fn span(&self) -> Span {
        match self {
            StringPart::Literal(LiteralStringPart { span, .. }) => *span,
            StringPart::Expression(ExpressionStringPart { span, .. }) => *span,
        }
    }
}

impl Spanned for ClassishMember {
    fn span(&self) -> Span {
        match self {
            ClassishMember::Constant(inner) => inner.span(),
            ClassishMember::TraitUsage(inner) => inner.span(),
            ClassishMember::Property(inner) => inner.span(),
            ClassishMember::VariableProperty(inner) => inner.span(),
            ClassishMember::AbstractMethod(inner) => inner.span(),
            ClassishMember::AbstractConstructor(inner) => inner.span(),
            ClassishMember::ConcreteMethod(inner) => inner.span(),
            ClassishMember::ConcreteConstructor(inner) => inner.span(),
            ClassishMember::Missing(inner) => inner.span(),
        }
    }
}

impl Spanned for IfStatementBody {
    fn span(&self) -> Span {
        match self {
            IfStatementBody::Statement(IfStatementBodyStatement { span, .. }) => *span,
            IfStatementBody::Block(IfStatementBodyBlock { span, .. }) => *span,
        }
    }
}

impl Spanned for DeclareBody {
    fn span(&self) -> Span {
        match self {
            DeclareBody::Noop(DeclareBodyNoop { span, .. }) => *span,
            DeclareBody::Braced(DeclareBodyBraced { span, .. }) => *span,
            DeclareBody::Expression(DeclareBodyExpression { span, .. }) => *span,
            DeclareBody::Block(DeclareBodyBlock { span, .. }) => *span,
        }
    }
}

impl Spanned for UnitEnumMember {
    fn span(&self) -> Span {
        match self {
            UnitEnumMember::Case(inner) => inner.span(),
            UnitEnumMember::Classish(inner) => inner.span(),
        }
    }
}

impl Spanned for BackedEnumMember {
    fn span(&self) -> Span {
        match self {
            BackedEnumMember::Case(inner) => inner.span(),
            BackedEnumMember::Classish(inner) => inner.span(),
        }
    }
}

impl Spanned for ForeachStatementIterator {
    fn span(&self) -> Span {
        match self {
            ForeachStatementIterator::Value(ForeachStatementIteratorValue { span, .. }) => *span,
            ForeachStatementIterator::KeyAndValue(ForeachStatementIteratorKeyAndValue {
                span,
                ..
            }) => *span,
        }
    }
}

impl Spanned for ForeachStatementBody {
    fn span(&self) -> Span {
        match self {
            ForeachStatementBody::Statement(inner) => inner.span(),
            ForeachStatementBody::Block(inner) => inner.span(),
        }
    }
}

impl Spanned for ForStatementBody {
    fn span(&self) -> Span {
        match self {
            ForStatementBody::Statement(inner) => inner.span(),
            ForStatementBody::Block(inner) => inner.span(),
        }
    }
}

impl Spanned for WhileStatementBody {
    fn span(&self) -> Span {
        match self {
            WhileStatementBody::Statement(inner) => inner.span(),
            WhileStatementBody::Block(inner) => inner.span(),
        }
    }
}

impl Spanned for Level {
    fn span(&self) -> Span {
        match self {
            Level::Literal(LiteralLevel { literal, .. }) => literal.span(),
            Level::Parenthesized(inner) => inner.span(),
        }
    }
}

impl Spanned for LiteralLevel {
    fn span(&self) -> Span {
        self.literal.span()
    }
}

impl Spanned for ArithmeticOperationKind {
    fn span(&self) -> Span {
        match self {
            ArithmeticOperationKind::Addition { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ArithmeticOperationKind::Subtraction { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ArithmeticOperationKind::Multiplication { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ArithmeticOperationKind::Division { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ArithmeticOperationKind::Modulo { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ArithmeticOperationKind::Exponentiation { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ArithmeticOperationKind::Negative { minus, right, .. } => {
                Span::combine(*minus, right.span)
            }
            ArithmeticOperationKind::Positive { plus, right, .. } => {
                Span::combine(*plus, right.span)
            }
            ArithmeticOperationKind::PreIncrement {
                increment, right, ..
            } => Span::combine(*increment, right.span),
            ArithmeticOperationKind::PostIncrement {
                left, increment, ..
            } => Span::combine(left.span, *increment),
            ArithmeticOperationKind::PreDecrement {
                decrement, right, ..
            } => Span::combine(*decrement, right.span),
            ArithmeticOperationKind::PostDecrement {
                left, decrement, ..
            } => Span::combine(left.span, *decrement),
        }
    }
}

impl Spanned for AssignmentOperationKind {
    fn span(&self) -> Span {
        match self {
            AssignmentOperationKind::Assign { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            AssignmentOperationKind::Addition { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            AssignmentOperationKind::Subtraction { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            AssignmentOperationKind::Multiplication { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            AssignmentOperationKind::Division { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            AssignmentOperationKind::Modulo { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            AssignmentOperationKind::Exponentiation { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            AssignmentOperationKind::Concat { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            AssignmentOperationKind::BitwiseAnd { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            AssignmentOperationKind::BitwiseOr { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            AssignmentOperationKind::BitwiseXor { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            AssignmentOperationKind::LeftShift { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            AssignmentOperationKind::RightShift { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            AssignmentOperationKind::Coalesce { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
        }
    }
}

impl Spanned for BitwiseOperationKind {
    fn span(&self) -> Span {
        match self {
            BitwiseOperationKind::And { left, right, .. } => Span::combine(left.span, right.span),
            BitwiseOperationKind::Or { left, right, .. } => Span::combine(left.span, right.span),
            BitwiseOperationKind::Xor { left, right, .. } => Span::combine(left.span, right.span),
            BitwiseOperationKind::LeftShift { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            BitwiseOperationKind::RightShift { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            BitwiseOperationKind::Not { not, right, .. } => Span::combine(*not, right.span),
        }
    }
}

impl Spanned for ComparisonOperationKind {
    fn span(&self) -> Span {
        match self {
            ComparisonOperationKind::Equal { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ComparisonOperationKind::Identical { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ComparisonOperationKind::NotEqual { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ComparisonOperationKind::AngledNotEqual { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ComparisonOperationKind::NotIdentical { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ComparisonOperationKind::LessThan { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ComparisonOperationKind::GreaterThan { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ComparisonOperationKind::LessThanOrEqual { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ComparisonOperationKind::GreaterThanOrEqual { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            ComparisonOperationKind::Spaceship { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
        }
    }
}

impl Spanned for LogicalOperationKind {
    fn span(&self) -> Span {
        match self {
            LogicalOperationKind::And { left, right, .. } => Span::combine(left.span, right.span),
            LogicalOperationKind::Or { left, right, .. } => Span::combine(left.span, right.span),
            LogicalOperationKind::Not { bang, right, .. } => Span::combine(*bang, right.span),
            LogicalOperationKind::LogicalAnd { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            LogicalOperationKind::LogicalOr { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
            LogicalOperationKind::LogicalXor { left, right, .. } => {
                Span::combine(left.span, right.span)
            }
        }
    }
}

impl Spanned for NameKind {
    fn span(&self) -> Span {
        match self {
            NameKind::Special(inner) => inner.span(),
            NameKind::Unresolved(inner) => inner.span(),
            NameKind::Resolved(inner) => inner.span(),
        }
    }
}

impl Spanned for PropertyEntryKind {
    fn span(&self) -> Span {
        match self {
            PropertyEntryKind::Uninitialized(inner) => inner.span(),
            PropertyEntryKind::Initialized(inner) => inner.span(),
        }
    }
}

impl Spanned for TraitUsageAdaptationKind {
    fn span(&self) -> Span {
        match self {
            TraitUsageAdaptationKind::Alias(inner) => inner.span(),
            TraitUsageAdaptationKind::Visibility(inner) => inner.span(),
            TraitUsageAdaptationKind::Precedence(inner) => inner.span(),
        }
    }
}

impl Spanned for CatchTypeKind {
    fn span(&self) -> Span {
        match self {
            CatchTypeKind::Identifier(inner) => inner.span(),
            CatchTypeKind::Union(inner) => inner.span(),
        }
    }
}

impl FunctionParameterList {
    pub fn iter(&self) -> impl Iterator<Item = &FunctionParameter> {
        self.parameters.iter()
    }
}

impl Spanned for CommentKind {
    fn span(&self) -> Span {
        match self {
            CommentKind::SingleLine(inner) => inner.span,
            CommentKind::MultiLine(inner) => inner.span,
            CommentKind::HashMark(inner) => inner.span,
            CommentKind::DocBlock(inner) => inner.span,
        }
    }
}

impl Spanned for DocBlockNode {
    fn span(&self) -> Span {
        match self {
            DocBlockNode::Text(inner) => inner.span,
            DocBlockNode::Tag(inner) => inner.span(),
        }
    }
}

impl Spanned for DocBlockTag {
    fn span(&self) -> Span {
        match self {
            DocBlockTag::Param(inner) => inner.span,
            DocBlockTag::Return(inner) => inner.span,
            DocBlockTag::Throws(inner) => inner.span,
            DocBlockTag::Var(inner) => inner.span,
            DocBlockTag::Property(inner) => inner.span,
            DocBlockTag::Method(inner) => inner.span,
            DocBlockTag::Template(inner) => inner.span,
            DocBlockTag::Extends(inner) => inner.span,
            DocBlockTag::Implements(inner) => inner.span,
            DocBlockTag::Uses(inner) => inner.span,
            DocBlockTag::Deprecated(inner) => inner.span,
            DocBlockTag::Generic(inner) => inner.span,
        }
    }
}