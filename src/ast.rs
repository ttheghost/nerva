use crate::common::{Span, Symbol};
use crate::arena::{NodeId, Arena};

type ExprId = NodeId<Expr>;
type StmtId = NodeId<Stmt>;
type TypeId = NodeId<Type>;
type ItemId = NodeId<Item>;

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pipeline,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
    Deref,
    AddressOf,
}

#[derive(Debug, Clone)]
pub enum AssignOp {
    Assign,
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(Symbol),
    Null
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Literal(Literal),
    Variable(Symbol),

    // Control flow
    Block {
        stmts: Vec<StmtId>,
        yield_expr: Option<ExprId>,
    },
    If {
        cond: ExprId,
        then_branch: ExprId,
        else_branch: Option<ExprId>,
    },
    Loop {
        body: ExprId,
    },
    While {
        cond: ExprId,
        body: ExprId,
        else_branch: Option<ExprId>,
    },
    For {
        var: Symbol,
        iter: ExprId,
        body: ExprId,
        else_branch: Option<ExprId>,
    },
    Return(Option<ExprId>),
    Break(Option<ExprId>),
    Continue,

    // Operations
    Binary {
        lhs: ExprId,
        op: BinaryOp,
        rhs: ExprId,
    },
    Assign {
        target: ExprId,
        op: AssignOp,
        value: ExprId,
    },
    Unary {
        op: UnaryOp,
        expr: ExprId,
    },
    Call {
        callee: ExprId,
        args: Vec<ExprId>,
    },
    MemberAccess {
        expr: ExprId,
        member: Symbol,
    },
    IndexAccess {
        expr: ExprId,
        index: ExprId,
    },

    // Error recovery
    Error,
}
#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    pub ty: Option<TypeId>,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    VarDecl {
        name: Symbol,
        init: ExprId,
        ty: Option<TypeId>,
        mutable: bool
    },
    Expr(ExprId),
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum TypeKind {}

#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Fn {
        name: Symbol,
        params: Vec<(Symbol, TypeId)>,
        ret_ty: Option<TypeId>,
        body: ExprId,
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
}

pub struct Ast {
    pub exprs: Arena<Expr>,
    pub stmts: Arena<Stmt>,
    pub types: Arena<Type>,
    pub items: Arena<Item>,
}

impl Ast {
    pub fn new(chunk_size: usize) -> Self {
        Ast {
            exprs: Arena::new(chunk_size),
            stmts: Arena::new(chunk_size),
            types: Arena::new(chunk_size),
            items: Arena::new(chunk_size),
        }
    }
}
