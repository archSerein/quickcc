use crate::lexer::lexer::Token;
use crate::utils::types::PhraseType;

use super::helper::{get_current_scope_num, look_up_symbol_table, print_symbol_table};

#[derive(Debug, Clone, PartialEq)]
pub struct StructType {
    pub id: String,
    pub field: Vec<VarDec>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    BaseType(String),
    CustomizedType(StructType),
}
#[derive(Debug, Clone, PartialEq)]
pub struct VarDec {
    pub var_type: Type,
    pub var_name: String,
    pub init: Option<ASTNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    // —— 程序、外部定义 ——
    Specifier(Box<Type>),
    Program {
        items: Vec<ASTNode>, // 全部顶层声明／定义
    },
    FuncDef {
        name: String,
        params: Vec<VarDec>,
        ret_type: Type,
        body: Box<ASTNode>, // Block
    },
    VarDecl(Vec<VarDec>),

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
    Args {
        args: Vec<Box<ASTNode>>,
    },
    Literal(Token),
    Ident(Token),
}

impl ASTNode {
    pub fn optimal(self) -> ASTNode {
        match self {
            // 冗余 BinaryOp: op == None，只有左节点，递归剥离
            ASTNode::BinaryOp {
                op: None,
                lhs,
                rhs: None,
            } => lhs.optimal(),
            // 常量折叠
            ASTNode::BinaryOp {
                op: Some(ref op),
                lhs,
                rhs: Some(rhs),
            } => {
                let lhs = lhs.optimal();
                let rhs = rhs.optimal();
                if let (ASTNode::Literal(l), ASTNode::Literal(r)) = (&lhs, &rhs) {
                    // 只支持整数
                    if let (Ok(lv), Ok(rv)) = (l.value.parse::<i32>(), r.value.parse::<i32>()) {
                        let result = match op.as_str() {
                            "+" => Some(lv + rv),
                            "-" => Some(lv - rv),
                            "*" => Some(lv * rv),
                            "/" => Some(lv / rv),
                            ">" => Some((lv > rv) as i32),
                            "<" => Some((lv < rv) as i32),
                            "==" => Some((lv == rv) as i32),
                            "!=" => Some((lv != rv) as i32),
                            _ => None,
                        };
                        if let Some(val) = result {
                            let token = Token {
                                types: l.types.clone(),
                                pos: l.pos,
                                value: val.to_string(),
                            };
                            return ASTNode::Literal(token);
                        }
                    }
                }
                ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs),
                    rhs: Some(Box::new(rhs)),
                }
            }
            // 其它 BinaryOp 递归处理
            ASTNode::BinaryOp { op, lhs, rhs } => ASTNode::BinaryOp {
                op,
                lhs: Box::new(lhs.optimal()),
                rhs: rhs.map(|n| Box::new(n.optimal())),
            },
            // 一元操作递归
            ASTNode::UnaryOp { op, expr } => ASTNode::UnaryOp {
                op,
                expr: Box::new(expr.optimal()),
            },
            // 变量声明，递归初始值
            ASTNode::VarDecl(var_decls) => ASTNode::VarDecl(
                var_decls
                    .into_iter()
                    .map(|mut v| {
                        v.init = v.init.map(|n| n.optimal());
                        v
                    })
                    .collect(),
            ),
            // 块递归
            ASTNode::Block { stmts } => ASTNode::Block {
                stmts: stmts.into_iter().map(|n| n.optimal()).collect(),
            },
            // If 分支递归
            ASTNode::If {
                cond,
                then_br,
                else_br,
            } => ASTNode::If {
                cond: Box::new(cond.optimal()),
                then_br: Box::new(then_br.optimal()),
                else_br: else_br.map(|b| Box::new(b.optimal())),
            },
            // While 递归
            ASTNode::While { cond, body } => ASTNode::While {
                cond: Box::new(cond.optimal()),
                body: Box::new(body.optimal()),
            },
            // Return 递归
            ASTNode::Return { expr } => ASTNode::Return {
                expr: expr.map(|e| Box::new(e.optimal())),
            },
            // 表达式语句递归
            ASTNode::ExprStmt { expr } => ASTNode::ExprStmt {
                expr: Box::new(expr.optimal()),
            },
            // Call 递归
            ASTNode::Call { func, args } => ASTNode::Call {
                func,
                args: args.into_iter().map(|a| a.optimal()).collect(),
            },
            // Args 递归
            ASTNode::Args { args } => ASTNode::Args {
                args: args.into_iter().map(|a| Box::new(a.optimal())).collect(),
            },
            // FuncDef 递归
            ASTNode::FuncDef {
                name,
                params,
                ret_type,
                body,
            } => ASTNode::FuncDef {
                name,
                params: params
                    .into_iter()
                    .map(|mut p| {
                        p.init = p.init.map(|n| n.optimal());
                        p
                    })
                    .collect(),
                ret_type,
                body: Box::new(body.optimal()),
            },
            // Program 递归
            ASTNode::Program { items } => ASTNode::Program {
                items: items.into_iter().map(|i| i.optimal()).collect(),
            },
            // 其它节点直接返回
            node => node,
        }
    }
    pub fn get_ast_type(node: &ASTNode) -> Option<Type> {
        match node {
            ASTNode::Ident(id) => {
                let symbol_info = look_up_symbol_table(&id.value);
                Some(symbol_info.unwrap().0.var_type)
            }
            ASTNode::Literal(literal) => match literal.types {
                PhraseType::Bool => Some(Type::BaseType(String::from("bool"))),
                PhraseType::String => Some(Type::BaseType(String::from("string"))),
                PhraseType::Oct | PhraseType::Dec | PhraseType::Hex => {
                    Some(Type::BaseType(String::from("int")))
                }
                PhraseType::Float => Some(Type::BaseType(String::from("float"))),
                PhraseType::Char => Some(Type::BaseType(String::from("char"))),
                _ => unreachable!(),
            },
            ASTNode::BinaryOp { lhs, .. } => {
                let lhs_ = lhs.as_ref();
                ASTNode::get_ast_type(lhs_)
            }
            ASTNode::UnaryOp { expr, .. } => {
                let expr_ = expr.as_ref();
                ASTNode::get_ast_type(expr_)
            }
            _ => None,
        }
    }
}
