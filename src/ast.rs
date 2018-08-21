#[derive(Debug, Clone)]
pub struct Statement;
#[derive(Debug, Clone)]
pub struct SimpleStmt;
#[derive(Debug, Clone)]
pub enum Test {
    Nil,
    IfExpr { cond: Box<Test>, alt: Box<Test> },
}

#[derive(Clone, Debug)]
pub enum Suite {
    Statements(Vec<Statement>),
    SimpleStmt(SimpleStmt),
}

#[derive(Debug, Clone)]
pub struct Slice;
#[derive(Debug, Clone)]
pub struct Call;

#[derive(Debug, Clone)]
pub struct Tuple(pub Vec<Expr>);

#[derive(Clone, Debug)]
pub enum Expr {
    // Binary
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
    In(Box<Expr>, Box<Expr>),
    NotIn(Box<Expr>, Box<Expr>),
    BitOr(Box<Expr>, Box<Expr>),
    BitAnd(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    DivFloor(Box<Expr>, Box<Expr>),

    // Unary
    Neg(Box<Expr>),
    Not(Box<Expr>),

    // Special
    Dot(Box<Expr>, String),
    Slice(Box<Expr>, Box<Slice>),
    Call(Box<Expr>, Box<Call>),

    // Operands
    Identifier(String),
    Int(i32),
    String(Vec<u8>),
    Tuple(Vec<Expr>),
    ListExpr(Vec<Expr>),
    ListComp,
    DictExpr,
    DictComp,
}
