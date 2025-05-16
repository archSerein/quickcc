#[derive(Debug, Clone)]
pub enum ASTNode {
    // —— 程序、外部定义 ——
    Program {
        items: Vec<ASTNode>, // 全部顶层声明／定义
    },
    FuncDef {
        name: String,
        params: Vec<(String, String)>,
        ret_type: String,
        body: Box<ASTNode>, // Block
    },
    VarDecl {
        name: String,
        var_type: String,
        init: Option<Box<ASTNode>>, // Optional init expression
    },

    // —— 语句 ——
    Block {
        stmts: Vec<ASTNode>,
    },
    If {
        cond: Box<ASTNode>,
        then_br: Box<ASTNode>,
        else_br: Option<Box<ASTNode>>,
    },
    While {
        cond: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    Return {
        expr: Option<Box<ASTNode>>,
    },
    ExprStmt {
        expr: Box<ASTNode>,
    },

    // —— 表达式 ——
    BinaryOp {
        op: Option<String>,
        lhs: Box<ASTNode>,
        rhs: Option<Box<ASTNode>>,
    },
    UnaryOp {
        op: String,
        expr: Box<ASTNode>,
    },
    Call {
        func: String,
        args: Vec<ASTNode>,
    },
    Literal(String),
    Ident(String),
}
