use pxp_token::Token;

#[derive(Debug, Clone)]
pub struct MagicConstantExpression {
    pub constant: MagicConstant,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum MagicConstant {
    __DIR__(Token),
    __LINE__(Token),
    __FILE__(Token),
    __FUNCTION__(Token),
    __CLASS__(Token),
    __TRAIT__(Token),
    __METHOD__(Token),
    __NAMESPACE__(Token),
    __COMPILER_HALT_OFFSET__(Token),
}