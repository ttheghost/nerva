use crate::common::{Span, Symbol};

#[derive(Debug, Clone)]
pub struct ExprId(pub u32);
#[derive(Debug, Clone)]
pub struct StmtId(pub u32);
#[derive(Debug, Clone)]
pub struct TypeId(pub u32);

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pipeline,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Literal(i128),
    Binary {
        lhs: ExprId,
        op: BinaryOp,
        rhs: ExprId,
    },
    Var(Symbol),
    Block {
        stmts: Vec<StmtId>,
        yield_expr: Option<ExprId>,
    },
    If {
        cond: ExprId,
        then_branch: ExprId,
        else_branch: Option<ExprId>,
    },
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

pub struct AstArena {
    pub exprs: Vec<Expr>,
    pub stmts: Vec<Stmt>,
}

impl AstArena {
    pub fn new() -> Self {
        AstArena {
            exprs: vec![],
            stmts: vec![],
        }
    }

    pub fn add_expr(&mut self, expr: Expr) -> ExprId {
        let idx = self.exprs.len();
        self.exprs.push(expr);
        ExprId(idx as u32)
    }

    pub fn add_stmt(&mut self, stmt: Stmt) -> StmtId {
        let idx = self.stmts.len();
        self.stmts.push(stmt);
        StmtId(idx as u32)
    }

    pub fn get_expr(&self, id: ExprId) -> &Expr {
        &self.exprs[id.0 as usize]
    }
    pub fn get_stmt(&self, id: StmtId) -> &Stmt {
        &self.stmts[id.0 as usize]
    }
}
