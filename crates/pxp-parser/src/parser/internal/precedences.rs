use crate::lexer::token::TokenKind;

pub enum Associativity {
    Non,
    Left,
    Right,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Precedence {
    Lowest,
    Print,
    Yield,
    YieldFrom,
    IncDec,
    KeyOr,
    KeyXor,
    KeyAnd,
    Assignment,
    Ternary,
    NullCoalesce,
    Or,
    And,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Equality,
    LtGt,
    Concat,
    BitShift,
    AddSub,
    MulDivMod,
    Bang,
    Instanceof,
    Prefix,
    Pow,
    CallDim,
    ObjectAccess,
    CloneOrNew,
}

impl Precedence {
    pub fn infix(kind: &TokenKind) -> Self {
        use TokenKind::*;

        match kind {
            Pow => Self::Pow,
            Instanceof => Self::Instanceof,
            Asterisk | Slash | Percent => Self::MulDivMod,
            Plus | Minus => Self::AddSub,
            LeftShift | RightShift => Self::BitShift,
            Dot => Self::Concat,
            LessThan | LessThanEquals | GreaterThan | GreaterThanEquals => Self::LtGt,
            DoubleEquals | BangEquals | TripleEquals | BangDoubleEquals | AngledLeftRight
            | Spaceship => Self::Equality,
            Ampersand => Self::BitwiseAnd,
            Caret => Self::BitwiseXor,
            Pipe => Self::BitwiseOr,
            BooleanAnd => Self::And,
            BooleanOr => Self::Or,
            DoubleQuestion => Self::NullCoalesce,
            Question | QuestionColon => Self::Ternary,
            Equals | PlusEquals | MinusEquals | AsteriskEquals | PowEquals | SlashEquals
            | DotEquals | AndEquals | DoubleQuestionEquals | PercentEquals | AmpersandEquals
            | PipeEquals | CaretEquals | LeftShiftEquals | RightShiftEquals => Self::Assignment,
            Yield => Self::Yield,
            LogicalAnd => Self::KeyAnd,
            LogicalOr => Self::KeyOr,
            LogicalXor => Self::KeyXor,
            _ => unimplemented!("precedence for op {:?}", kind),
        }
    }

    pub fn postfix(kind: &TokenKind) -> Self {
        use TokenKind::*;

        match kind {
            DoubleQuestion => Self::NullCoalesce,
            Increment | Decrement => Self::IncDec,
            LeftParen | LeftBracket => Self::CallDim,
            Arrow | QuestionArrow | DoubleColon => Self::ObjectAccess,
            _ => unimplemented!("postfix precedence for op {:?}", kind),
        }
    }

    pub fn associativity(&self) -> Option<Associativity> {
        Some(match self {
            Self::Instanceof
            | Self::MulDivMod
            | Self::AddSub
            | Self::BitShift
            | Self::Concat
            | Self::BitwiseAnd
            | Self::BitwiseOr
            | Self::BitwiseXor
            | Self::And
            | Self::Or
            | Self::KeyAnd
            | Self::KeyOr
            | Self::KeyXor => Associativity::Left,
            Self::Pow | Self::NullCoalesce | Self::Assignment => Associativity::Right,
            Self::Ternary | Self::Equality | Self::LtGt => Associativity::Non,
            _ => return None,
        })
    }
}
