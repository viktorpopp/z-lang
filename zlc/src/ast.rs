#[derive(Debug)]
pub struct Ident {
    pub inner: String,
}

impl Ident {
    pub fn new(s: impl Into<String>) -> Self {
        Self { inner: s.into() }
    }
}

#[derive(Debug)]
pub struct IntLit {
    pub inner: String, // idk
}

impl IntLit {
    pub fn new(s: impl Into<String>) -> Self {
        Self { inner: s.into() }
    }
}

#[derive(Debug)]
pub struct Package {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Function(Func),
}

#[derive(Debug)]
pub struct Func {
    pub sym: Ident,
    pub ret: Option<FuncRetType>,
    pub body: BlockExpr,
}

#[derive(Debug)]
pub struct FuncRetType {
    pub r#type: Type,
}

#[derive(Debug)]
pub struct Type {
    pub path: TypePath,
}

#[derive(Debug)]
pub struct TypePath {
    pub seg: TypePathSeg,
}

#[derive(Debug)]
pub struct TypePathSeg {
    pub iseg: PathIdentSeg,
}

#[derive(Debug)]
pub struct PathIdentSeg {
    pub ident: Ident,
}

#[derive(Debug)]
pub struct BlockExpr {
    pub stmts: Option<Stmts>,
}

#[derive(Debug)]
pub struct Stmts {
    pub inner: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Empty,
    ExprStmt(ExprStmt),
}

#[derive(Debug)]
pub enum ExprStmt {
    WithoutBlock(ExprWithoutBlock),
}

#[derive(Debug)]
pub enum ExprWithoutBlock {
    LitExpr(LitExpr),
    ReturnExpr(Box<ReturnExpr>),
}

#[derive(Debug)]
pub struct ReturnExpr {
    pub expr: Option<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    ExprWithoutBlock(ExprWithoutBlock),
}

#[derive(Debug)]
pub struct LitExpr {
    pub lit: IntLit,
}
