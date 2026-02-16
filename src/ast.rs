use crate::arena::{Arena, NodeId};
use crate::common::{Span, Symbol};

type ExprId = NodeId<Expr>;
type StmtId = NodeId<Stmt>;
type ItemId = NodeId<Item>;
type TypeSpecId = NodeId<TypeSpec>;
type PatternId = NodeId<Pattern>;

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
    Deref,
    Ref,
    AddressOf,
}

#[derive(Debug, Clone)]
pub enum AssignOp {
    Assign,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Literal(Literal),
    Identifier(Symbol),

    Paren(ExprId),

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
    Match {
        target: ExprId,
        cases: Vec<MatchCase>,
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
        binding: Symbol,
        iterable: ExprId,
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
    Pipeline {
        lhs: ExprId,
        rhs: ExprId,
    },
    Cast {
        target: ExprId,
        target_type: TypeSpecId,
    },
    Unary {
        op: UnaryOp,
        operand: ExprId,
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
pub struct MatchCase {
    pub pattern: PatternId,
    pub body: ExprId,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    pub ty: Option<TypeSpecId>,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    VarDecl {
        is_mutable: bool,
        name: Symbol,
        ty: Option<TypeSpecId>,
        init: VarInit,
    },
    Defer(ExprId),
    Expr(ExprId),
}

#[derive(Debug, Clone)]
pub enum VarInit {
    Expr(ExprId),
    Undefined,
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    Identifier(Symbol),
    Wildcard,
}

#[derive(Debug, Clone)]
pub enum TypeSpecKind {
    // This type I will treat primitive types like unresolved ones
    // and inject them later
    Named(Symbol),
    Pointer(TypeSpecId),
    Reference(TypeSpecId),
    Optional(TypeSpecId),
    Array {
        size: ExprId,
        elem_ty: TypeSpecId,
    },
    Slice(TypeSpecId),
    Fn {
        param: Vec<TypeSpecId>,
        return_ty: TypeSpecId,
    },
    Paren(TypeSpecId),
}

#[derive(Debug, Clone)]
pub struct TypeSpec {
    pub kind: TypeSpecKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    FnDecl {
        name: Symbol,
        params: Vec<Param>,
        ret_ty: Option<TypeSpecId>,
        body: ExprId,
    },
    StructDecl {
        name: Symbol,
        fields: Vec<StructField>,
    },
    EnumDecl {
        name: Symbol,
        backing_ty: Option<TypeSpecId>,
        variants: Vec<EnumVariant>,
    },
    UnionDecl {
        name: Symbol,
        variants: Vec<UnionVariant>,
    },
    ImplDecl {
        self_ty: TypeSpecId,
        methods: Vec<ItemId>,
    },
    ConstDecl {
        name: Symbol,
        ty: TypeSpecId,
        expr: ExprId,
    },
    ExternDecl {
        api: String,
        declarations: Vec<FnSig>,
    },
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: Symbol,
    pub ty: TypeSpecId,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: Symbol,
    pub ty: TypeSpecId,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: Symbol,
    pub value: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct UnionVariant {
    pub name: Symbol,
    pub data: Option<UnionVariantData>,
}

#[derive(Debug, Clone)]
pub enum UnionVariantData {
    Tuple(Vec<TypeSpecId>),
    Struct(Vec<StructField>),
}

#[derive(Debug, Clone)]
pub struct FnSig {
    pub name: Symbol,
    pub params: Vec<Param>,
    pub return_ty: Option<TypeSpecId>,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
}

pub struct Ast {
    pub exprs: Arena<Expr>,
    pub stmts: Arena<Stmt>,
    pub type_specs: Arena<TypeSpec>,
    pub items: Arena<Item>,
    pub patterns: Arena<Pattern>,
}

impl Ast {
    pub fn new(chunk_size: usize) -> Self {
        Ast {
            exprs: Arena::new(chunk_size),
            stmts: Arena::new(chunk_size),
            type_specs: Arena::new(chunk_size),
            items: Arena::new(chunk_size),
            patterns: Arena::new(chunk_size),
        }
    }
}
