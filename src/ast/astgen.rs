use super::types::ASTNode;
use crate::parser::parse::CSTNode;
use std::vec;

pub fn ast_gen(cst: &Vec<CSTNode>) -> Vec<ASTNode> {
    let mut ast: Vec<ASTNode> = vec![];
    for node in cst {
        let ast_node = ASTNode::from_cst(node);
        ast.push(ast_node);
    }
    ast
}

impl ASTNode {
    pub fn from_cst(node: &CSTNode) -> ASTNode {
        match node {
            CSTNode::Program(boxed) => {
                let mut items = Vec::new();
                collect_extdefs(boxed, &mut items);
                ASTNode::Program { items }
            }
            CSTNode::ExtDef {
                spec,
                fun_dec: Some(fun),
                compst: Some(body),
                ..
            } => {
                let (ret_type, _) = extract_spec(spec);
                let (name, params) = extract_fundec(fun);
                let body_block = ASTNode::from_cst(&**body);
                ASTNode::FuncDef {
                    name,
                    params,
                    ret_type,
                    body: Box::new(body_block),
                }
            }
            CSTNode::ExtDecList {
                var_dec,
                ext_dec_list,
                ..
            } => {
                // 顶层变量声明
                let mut decls = Vec::new();
                collect_vardecls(node, &mut decls);
                // 如果只有一个，可以直接返回 VarDecl，否则包在 Block 里
                ASTNode::Block { stmts: decls }
            }
            CSTNode::CompSt {
                def_list,
                stmt_list,
                ..
            } => {
                let mut stmts = Vec::new();
                if let Some(defs) = def_list {
                    collect_local_decls(defs, &mut stmts);
                }
                collect_stmts(stmt_list, &mut stmts);
                ASTNode::Block { stmts }
            }
            CSTNode::MatchedStmt {
                if_stmt: Some(_),
                expression: Some(cond),
                matched_stmt_fore: Some(then_),
                matched_stmt_back: else_opt,
                ..
            } => {
                let then_node = ASTNode::from_cst(&**then_);
                let else_node = else_opt.as_ref().map(|b| ASTNode::from_cst(&**b));
                ASTNode::If {
                    cond: Box::new(ASTNode::from_cst(&**cond)),
                    then_br: Box::new(then_node),
                    else_br: else_node.map(Box::new),
                }
            }
            CSTNode::UnMatchedStmt {
                while_stmt: Some(_),
                expression: Some(cond),
                stmt: Some(body),
                ..
            } => ASTNode::While {
                cond: Box::new(ASTNode::from_cst(&**cond)),
                body: Box::new(ASTNode::from_cst(&**body)),
            },
            CSTNode::NormalStmt {
                return_stmt: Some(_),
                expression: expr_opt,
                ..
            } => ASTNode::Return {
                expr: expr_opt.as_ref().map(|e| Box::new(ASTNode::from_cst(&**e))),
            },
            CSTNode::NormalStmt {
                expression: Some(expr),
                ..
            } => ASTNode::ExprStmt {
                expr: Box::new(ASTNode::from_cst(&**expr)),
            },
            CSTNode::Assign {
                logical_or,
                assign_prime: None,
            } => {
                // 单纯表达式
                ASTNode::from_cst(&**logical_or)
            }
            CSTNode::AssignPrime {
                op,
                logical_or,
                assign_prime: Some(next),
            } if op == "=" => {
                // 赋值链式： a = b = c 处理成二元树
                let lhs = ASTNode::from_cst(&**logical_or);
                let rhs = ASTNode::from_cst(&**next); // 递归拆 assign_prime
                ASTNode::BinaryOp {
                    op: String::from("="),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            }
            // … 其他表达式优先级节点：LogicalOr, LogicalAnd, Equality, Comparison, Term, Factor, Unary, Primary…
            CSTNode::Primary {
                symbol: Some(id), ..
            } => ASTNode::Ident(id.clone()),
            CSTNode::Primary {
                fun_call: Some(fcall),
                ..
            } => {
                let call = &**fcall;
                // let args = extract_arguments(call.arguments.as_deref());
                ASTNode::Call {
                    func: String::from("main"),
                    args: Vec::new(),
                }
            }
            // 文本字面量
            CSTNode::VarDec {
                literal: Some(lit), ..
            } => ASTNode::Literal(lit.clone()),
            other => {
                unimplemented!("未处理的 CSTNode: {:?}", other)
            }
        }
    }
}

// 辅助函数：遍历列表、拆 spec/fundec/arguments 等
fn collect_extdefs(node: &CSTNode, items: &mut Vec<ASTNode>) {
    match node {
        CSTNode::ExtDefList {
            ext_def,
            ext_def_list: Some(list),
        } => {
            items.push(ASTNode::from_cst(ext_def));
            collect_extdefs(list, items);
        }
        _ => {
            unreachable!();
        }
    }
}
fn extract_spec(spec: &CSTNode) -> (String, Option<String>) {
    todo!()
}
fn extract_fundec(fun: &CSTNode) -> (String, Vec<(String, String)>) {
    /* … */
    todo!()
}
fn extract_arguments(arg_node: Option<&CSTNode>) -> Vec<ASTNode> {
    /* … */
    todo!()
}
fn collect_local_decls(node: &CSTNode, out: &mut Vec<ASTNode>) {
    /* … */
    todo!()
}
fn collect_stmts(node: &CSTNode, out: &mut Vec<ASTNode>) {
    /* … */
    todo!()
}
fn collect_vardecls(node: &CSTNode, out: &mut Vec<ASTNode>) {
    /* … */
    todo!()
}
