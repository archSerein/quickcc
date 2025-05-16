use super::types::ASTNode;
use crate::{parser::parse::CSTNode, utils::helper::symbol_is_literal};
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
                ext_dec_list: Some(list),
                ..
                // sepa, 分隔符号是否需要记录到 ast 中
            } => {
                let (ret_type, _) = extract_spec(spec);
                let (name, params) = extract_fundec(fun);
                let body_block = ASTNode::from_cst(body);
                let mut items = Vec::new();
                collect_extdecs(list, &mut items);
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
            CSTNode::VarList { para_dec, sepa, var_list } => {
                if let Some(value) = sepa {
                    let list = var_list.unwrap();
                    collexct_paradec(list, &mut items);
                }
            }
            CSTNode::Assign {
                logical_or,
                assign_prime,
            } => {
                let lhs = ASTNode::from_cst(logical_or);
                if let Some(prime) = assign_prime {
                    let ast_node = extract_assign(&lhs, prime);
                    ast_node
                } else {
                    ASTNode::BinaryOp { op: None, lhs: Box::new(lhs), rhs: None }
                }
            }
            CSTNode::LogicalOr { logical_and, logical_or_prime } => {
                let lhs = ASTNode::from_cst(logical_and);
                if let Some(prime) = logical_or_prime {
                    let ast_node = extract_logical_or(&lhs, prime);
                    ast_node
                } else {
                    ASTNode::BinaryOp { op: None, lhs: Box::new(lhs), rhs: None }
                }
            }
            CSTNode::LogicalAnd { equality, logical_and_prime } => {
                let lhs = ASTNode::from_cst(equality);
                if let Some(prime) = logical_and_prime {
                    let ast_node = extract_logical_and(&lhs, prime);
                    ast_node
                } else {
                    ASTNode::BinaryOp { op: None, lhs: Box::new(lhs), rhs: None }
                }
            }
            CSTNode::Equality { comparison, equality_prime } => {
                let lhs = ASTNode::from_cst(comparison);
                if let Some(prime) = equality_prime {
                    let ast_node = extract_equality(&lhs, prime);
                    ast_node
                } else {
                    ASTNode::BinaryOp { op: None, lhs: Box::new(lhs), rhs: None }
                }
            }
            CSTNode::Comparison { term, comparison_prime } => {
                let lhs = ASTNode::from_cst(term);
                if let Some(prime) = comparison_prime {
                    let ast_node = extract_comparison(&lhs, prime);
                    ast_node
                } else {
                    ASTNode::BinaryOp { op: None, lhs: Box::new(lhs), rhs: None }
                }
            }
            CSTNode::Term {factor, term_prime } => {
                let lhs = ASTNode::from_cst(factor);
                if let Some(prime) = term_prime {
                    let ast_node = extract_term(&lhs, prime);
                    ast_node
                } else {
                    ASTNode::BinaryOp { op: None, lhs: Box::new(lhs), rhs: None }
                }
            }
            CSTNode::Factor { unary, factor_prime } => {
                let lhs = ASTNode::from_cst(unary);
                if let Some(prime) = factor_prime {
                    let ast_node = extract_factor(&lhs, prime);
                    ast_node
                } else {
                    ASTNode::BinaryOp { op: None, lhs: Box::new(lhs), rhs: None }
                }
            }
            CSTNode::Unary { op, unary } => {
                if let Some(value) = op {
                    let expr = ASTNode::from_cst(unary);
                    ASTNode::UnaryOp { op: value.clone(), expr: Box::new(expr) }
                } else {
                    ASTNode::from_cst(unary)
                }
            }
            CSTNode::Primary {
                symbol: Some(value), ..
            } => {
                if symbol_is_literal(value) {
                    ASTNode::Literal(value.clone())
                } else {
                    ASTNode::Ident(value.clone())
                }
            }
            CSTNode::Primary {
                fun_call: Some(fcall),
                ..
            } => {
                extract_fcall(fcall)
            }
            CSTNode::Expression(expr) => {
                ASTNode::from_cst(expr)
            }
            other => {
                unimplemented!("未处理的 CSTNode: {:?}", other)
            }
        }
    }
}

fn collect_extdefs(node: &CSTNode, items: &mut Vec<ASTNode>) {
    match node {
        CSTNode::ExtDefList {
            ext_def,
            ext_def_list,
        } => {
            items.push(ASTNode::from_cst(ext_def));
            if let Some(list) = ext_def_list {
                collect_extdefs(list, items);
            } else {
                // do nothing
            }
        }
        _ => {
            unreachable!();
        }
    }
}
fn collect_extdecs(node: &CSTNode, items: &mut Vec<ASTNode>) {
    todo!()
}
fn extract_struct_sepc(node: &CSTNode) -> Vec<String> {
    match node {
        CSTNode::StructSpecifier {
            struct_type,
            id,
            lc,
            rc,
            def_list,
        } => {
            let mut items: Vec<String> = vec![];
            items.push(struct_type.clone());
            if let Some(value) = id {
                items.push(value.clone())
            }
            if let Some(value) = lc {
                items.push(value.clone())
            }
            if let Some(list) = def_list {
                collect_defs(list, &mut items);
            }
            if let Some(value) = rc {
                items.push(value.clone())
            }
            items
        }
        _ => unreachable!(),
    }
}
fn extract_spec(spec: &CSTNode) -> (Option<String>, Option<Vec<String>>) {
    match spec {
        CSTNode::Specifier {
            specifier_type,
            struct_specifier,
        } => {
            if let Some(value) = specifier_type {
                (Some(value.clone()), None)
            } else if let Some(struct_value) = struct_specifier {
                let ret = extract_struct_sepc(struct_value);
                (None, Some(ret))
            } else {
                // 不应该出现既不是自定义的结构体
                // 也不是基本的类型
                unreachable!()
            }
        }
        _ => unreachable!(),
    }
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
fn collect_defs(node: &CSTNode, items: &mut Vec<String>) {
    match node {
        CSTNode::DefList { def, def_list } => {
            items.push(ASTNode::from_cst(ext_def));
            if let Some(list) = ext_def_list {
                collect_extdefs(list, items);
            } else {
                // do nothing
            }
        }
        _ => {
            unreachable!();
        }
    }
}
// fn extract_def(node: &CSTNode, items: &mut Vec<>)
// 由于文法的错误最多只能接受两个参数
// TODO:
// WARN:
fn colloct_arguments(node: &CSTNode) -> Vec<ASTNode> {
    match node {
        CSTNode::Arguments {
            arguments_tail,
            expression,
        } => {
            let mut args: Vec<ASTNode> = vec![];
            args.push(ASTNode::from_cst(&expression));
            if let Some(next_arg) = arguments_tail {
                args.extend(colloct_arguments(next_arg));
            }
            args
        }
        CSTNode::ArgumentsTail { expression, .. } => {
            let mut args: Vec<ASTNode> = vec![];
            args.push(ASTNode::from_cst(&expression));
            args
        }
        _ => unreachable!(),
    }
}
fn extract_fcall(fcall: &CSTNode) -> ASTNode {
    match fcall {
        CSTNode::FunCall { id, arguments, .. } => {
            if let Some(args) = arguments {
                ASTNode::Call {
                    func: id.clone(),
                    args: colloct_arguments(args),
                }
            } else {
                ASTNode::Call {
                    func: id.clone(),
                    args: vec![],
                }
            }
        }
        _ => unreachable!(),
    }
}
fn extract_assign(lhs: &ASTNode, node: &CSTNode) -> ASTNode {
    match node {
        CSTNode::AssignPrime {
            op,
            logical_or,
            assign_prime,
        } => {
            if let Some(prime) = assign_prime {
                let rhs = extract_assign(&ASTNode::from_cst(logical_or), prime);
                ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                }
            } else {
                let rhs = ASTNode::from_cst(logical_or);
                ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                }
            }
        }
        _ => unreachable!(),
    }
}
fn extract_logical_or(lhs: &ASTNode, node: &CSTNode) -> ASTNode {
    match node {
        CSTNode::LogicalOrPrime {
            op,
            logical_and,
            logical_or_prime,
        } => {
            if let Some(prime) = logical_or_prime {
                let rhs = ASTNode::from_cst(logical_and);
                let ast_node = ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                };
                extract_logical_or(&ast_node, prime)
            } else {
                ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(ASTNode::from_cst(logical_and))),
                }
            }
        }
        _ => unreachable!(),
    }
}
fn extract_logical_and(lhs: &ASTNode, node: &CSTNode) -> ASTNode {
    match node {
        CSTNode::LogicalAndPrime {
            op,
            equality,
            logical_and_prime,
        } => {
            if let Some(prime) = logical_and_prime {
                let rhs = ASTNode::from_cst(equality);
                let ast_node = ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                };
                extract_logical_or(&ast_node, prime)
            } else {
                ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(ASTNode::from_cst(equality))),
                }
            }
        }
        _ => unreachable!(),
    }
}
fn extract_equality(lhs: &ASTNode, node: &CSTNode) -> ASTNode {
    match node {
        CSTNode::EqualityPrime {
            op,
            comparison,
            equality_prime,
        } => {
            if let Some(prime) = equality_prime {
                let rhs = ASTNode::from_cst(comparison);
                let ast_node = ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                };
                extract_logical_or(&ast_node, prime)
            } else {
                ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(ASTNode::from_cst(comparison))),
                }
            }
        }
        _ => unreachable!(),
    }
}
fn extract_comparison(lhs: &ASTNode, node: &CSTNode) -> ASTNode {
    match node {
        CSTNode::ComparisonPrime {
            op,
            term,
            comparison_prime,
        } => {
            if let Some(prime) = comparison_prime {
                let rhs = ASTNode::from_cst(term);
                let ast_node = ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                };
                extract_logical_or(&ast_node, prime)
            } else {
                ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(ASTNode::from_cst(term))),
                }
            }
        }
        _ => unreachable!(),
    }
}
fn extract_term(lhs: &ASTNode, node: &CSTNode) -> ASTNode {
    match node {
        CSTNode::TermPrime {
            op,
            factor,
            term_prime,
        } => {
            if let Some(prime) = term_prime {
                let rhs = ASTNode::from_cst(factor);
                let ast_node = ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                };
                extract_logical_or(&ast_node, prime)
            } else {
                ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(ASTNode::from_cst(factor))),
                }
            }
        }
        _ => unreachable!(),
    }
}
fn extract_factor(lhs: &ASTNode, node: &CSTNode) -> ASTNode {
    match node {
        CSTNode::FactorPrime {
            op,
            unary,
            factor_prime,
        } => {
            if let Some(prime) = factor_prime {
                let rhs = ASTNode::from_cst(unary);
                let ast_node = ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                };
                extract_logical_or(&ast_node, prime)
            } else {
                ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(ASTNode::from_cst(unary))),
                }
            }
        }
        _ => unreachable!(),
    }
}
