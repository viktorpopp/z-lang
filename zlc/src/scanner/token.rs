#[derive(Copy, Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Copy, Clone, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum TokenKind {
    // Single-character tokens
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Semicolon,

    // Literals
    Literal(Literal),
    Identifier,

    // Keywords
    Fn,
    Return,

    // Miscellaneous
    EndOfFile,
}

#[derive(Copy, Clone, Debug)]
pub struct Literal {
    pub kind: LiteralKind,
}

#[derive(Copy, Clone, Debug)]
pub enum LiteralKind {
    Integer,
}
