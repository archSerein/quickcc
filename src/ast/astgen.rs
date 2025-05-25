use super::{
    helper::{
        build_symbol_table, check_entry_func, check_types, get_current_func, get_current_scope_num,
        print_symbol_table, set_current_func, update_current_scope_num,
    },
    types::{ASTNode, StructType, Type, VarDec},
};
use crate::{parser::parse::CSTNode, utils::helper::symbol_is_literal};
use std::vec;

pub fn ast_gen(cst: &Vec<CSTNode>) -> Vec<ASTNode> {
    let mut ast: Vec<ASTNode> = vec![];
    for node in cst {
        let ast_node = ASTNode::from_cst(node);
        ast.push(ast_node.optimal());
    }
    if !check_entry_func() {
        println!("No entry function main is defined");
        unreachable!()
    }
    print_symbol_table();
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
                fun_dec,
                compst,
                ext_dec_list,
                ..
                // sepa, 分隔符号是否需要记录到 ast 中
            } => {
                let ret_type = extract_spec(spec);
                match (fun_dec, compst, ext_dec_list) {
                    (Some(func), Some(compst), None) => {
                        let (name, params) = extract_fundec(func);
                        let var_info = VarDec {
                            var_type: ret_type.clone(),
                            var_name: name.clone(),
                            init: None
                        };
                        build_symbol_table(get_current_func(),&var_info, get_current_scope_num(), Some(params.clone()));
                        let body_block = ASTNode::from_cst(compst);
                        ASTNode::FuncDef {
                            name,
                            params,
                            ret_type,
                            body: Box::new(body_block),
                        }
                    }
                    (None, None, Some(list)) => {
                        let mut items = Vec::new();
                        collect_extdecs(ret_type.clone(),list, &mut items);
                        ASTNode::VarDecl(items)
                    }
                    (None, None, None) => {
                        ASTNode::Specifier(Box::new(ret_type))
                    }
                    _ => unreachable!()
                }
            }
            CSTNode::CompSt {
                def_list,
                stmt_list,
                ..
            } => {
                update_current_scope_num(get_current_scope_num()+1);
                let mut stmts = Vec::new();
                if let Some(defs) = def_list {
                    let mut list = Vec::new();
                    collect_defs(defs, &mut list);
                    stmts.push(ASTNode::VarDecl(list));
                }
                collect_stmts(stmt_list, &mut stmts);
                update_current_scope_num(get_current_scope_num()-1);
                ASTNode::Block { stmts }
            }
            CSTNode::MatchedStmt {
                if_stmt,
                while_stmt,
                expression,
                matched_stmt_fore,
                matched_stmt_back,
                normal_stmt,
                ..
            } => {
                let cond = expression.as_ref().map(|e| Box::new(ASTNode::from_cst(e)));
                let then_br = matched_stmt_fore.as_ref().map(|e| Box::new(ASTNode::from_cst(e)));
                let else_br = matched_stmt_back.as_ref().map(|e| Box::new(ASTNode::from_cst(e)));
                match (if_stmt, while_stmt, normal_stmt) {
                    (Some(_), None, None) => {
                        let cond_ = cond.unwrap();
                        let then_br_ = then_br.unwrap();
                        ASTNode::If { cond: cond_, then_br: then_br_, else_br }
                    }
                    (None, Some(_), None) => {
                        let cond_ = cond.unwrap();
                        let body_ = then_br.unwrap();
                        ASTNode::While { cond: cond_, body: body_ }
                    }
                    (None, None, Some(stmt)) => {
                        ASTNode::from_cst(stmt)
                    }
                    _ => {
                        println!("不正确的 MatchedStmt");
                        unreachable!()
                    }
                }
            }
            CSTNode::UnMatchedStmt {
                while_stmt,
                expression,
                stmt,
                if_stmt,
                else_stmt,
                matched_stmt,
                unmatched_stmt,
                ..
            } => {
                let cond = expression.as_ref().map(|e| Box::new(ASTNode::from_cst(e)));
                if while_stmt.is_some() {
                    let body = ASTNode::from_cst(unmatched_stmt.clone().unwrap().as_ref());
                    ASTNode::While { cond: cond.unwrap(), body: Box::new(body) }
                } else if if_stmt.is_some() && else_stmt.is_some() {
                    let then_br = matched_stmt.as_ref().map(|e| Box::new(ASTNode::from_cst(e)));
                    let else_br = unmatched_stmt.as_ref().map(|e| Box::new(ASTNode::from_cst(e)));
                    ASTNode::If { cond: cond.unwrap(), then_br: then_br.unwrap(), else_br }
                } else if if_stmt.is_some() && !else_stmt.is_some() {
                    let then_br = stmt.as_ref().map(|e| Box::new(ASTNode::from_cst(e)));
                    ASTNode::If { cond: cond.unwrap(), then_br: then_br.unwrap(), else_br: None }
                } else {
                    unreachable!()
                }
            },
            CSTNode::NormalStmt {
                return_stmt,
                expression,
                compst,
                ..
            } => {
                if return_stmt.is_some() {
                    let expr = expression.as_ref().map(|e| Box::new(ASTNode::from_cst(e)));
                    ASTNode::Return { expr }
                } else if let Some(value) = expression {
                    ASTNode::from_cst(value.as_ref())
                } else {
                    let value = compst.clone().unwrap();
                    ASTNode::from_cst(value.as_ref())
                }
            },
            CSTNode::Def { spec, dec_list, .. } => {
                let var_type = extract_spec(spec);
                let var_decs = collect_decs(var_type, dec_list);
                ASTNode::VarDecl(var_decs)
            }
            CSTNode::Assign {
                logical_or,
                assign_prime,
            } => {
                let lhs = ASTNode::from_cst(logical_or);
                if let Some(prime) = assign_prime {
                    extract_assign(&lhs, prime)
                } else {
                    ASTNode::BinaryOp { op: None, lhs: Box::new(lhs), rhs: None }
                }
            }
            CSTNode::LogicalOr { logical_and, logical_or_prime } => {
                let lhs = ASTNode::from_cst(logical_and);
                if let Some(prime) = logical_or_prime {
                    extract_logical_or(&lhs, prime)
                } else {
                    ASTNode::BinaryOp { op: None, lhs: Box::new(lhs), rhs: None }
                }
            }
            CSTNode::LogicalAnd { equality, logical_and_prime } => {
                let lhs = ASTNode::from_cst(equality);
                if let Some(prime) = logical_and_prime {
                    extract_logical_and(&lhs, prime)
                } else {
                    ASTNode::BinaryOp { op: None, lhs: Box::new(lhs), rhs: None }
                }
            }
            CSTNode::Equality { comparison, equality_prime } => {
                let lhs = ASTNode::from_cst(comparison);
                if let Some(prime) = equality_prime {
                    extract_equality(&lhs, prime)
                } else {
                    ASTNode::BinaryOp { op: None, lhs: Box::new(lhs), rhs: None }
                }
            }
            CSTNode::Comparison { term, comparison_prime } => {
                let lhs = ASTNode::from_cst(term);
                if let Some(prime) = comparison_prime {
                    extract_comparison(&lhs, prime)
                } else {
                    ASTNode::BinaryOp { op: None, lhs: Box::new(lhs), rhs: None }
                }
            }
            CSTNode::Term {factor, term_prime } => {
                let lhs = ASTNode::from_cst(factor);
                if let Some(prime) = term_prime {
                    extract_term(&lhs, prime)
                } else {
                    ASTNode::BinaryOp { op: None, lhs: Box::new(lhs), rhs: None }
                }
            }
            CSTNode::Factor { unary, factor_prime } => {
                let lhs = ASTNode::from_cst(unary);
                if let Some(prime) = factor_prime {
                    extract_factor(&lhs, prime)
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
                if symbol_is_literal(&value.types) {
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

// TODO:
// BUG:
fn collect_decs(var_type: Type, node: &CSTNode) -> Vec<VarDec> {
    match node {
        CSTNode::DecList { dec, dec_list, .. } => {
            let mut items: Vec<VarDec> = vec![];
            let var_dec = extract_dec(var_type.clone(), dec);
            items.push(var_dec.clone());
            build_symbol_table(get_current_func(), &var_dec, get_current_scope_num(), None);
            if let Some(list) = dec_list {
                let var_list = collect_decs(var_type.clone(), list);
                items.extend(var_list);
                items
            } else {
                items
            }
        }
        _ => unreachable!(),
    }
}

fn collect_paradec(node: &CSTNode, items: &mut Vec<VarDec>) {
    match node {
        CSTNode::VarList {
            para_dec, var_list, ..
        } => {
            let param = extract_param(para_dec);
            build_symbol_table(
                get_current_func(),
                &param,
                get_current_scope_num() + 1,
                None,
            );
            items.push(param);
            if let Some(list) = var_list {
                collect_paradec(list, items);
            }
        }
        _ => unreachable!(),
    }
}
// TODO:
fn collect_extdecs(var_type: Type, node: &CSTNode, items: &mut Vec<VarDec>) {
    match node {
        CSTNode::ExtDecList {
            var_dec,
            ext_dec_list,
            ..
        } => {
            let var_dec = extract_dec(var_type.clone(), var_dec);
            items.push(var_dec);
            if let Some(list) = ext_dec_list {
                collect_extdecs(var_type.clone(), list, items);
            } else {
                // nothing
            }
        }
        _ => {
            unreachable!()
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
fn extract_dec(var_type: Type, node: &CSTNode) -> VarDec {
    match node {
        CSTNode::Dec {
            var_dec,
            op,
            expression,
        } => {
            let var_name = extract_vardec(var_dec);
            if let Some(value) = op {
                if value.eq("=") {
                    let node = expression.clone().unwrap();
                    let init = ASTNode::from_cst(&node.clone());
                    VarDec {
                        var_type,
                        var_name,
                        init: Some(init),
                    }
                } else {
                    println!("unexpected op");
                    VarDec {
                        var_type: Type::BaseType(String::new()),
                        var_name: String::new(),
                        init: None,
                    }
                }
            } else {
                VarDec {
                    var_type,
                    var_name,
                    init: None,
                }
            }
        }
        _ => unreachable!(),
    }
}
fn extract_vardec(node: &CSTNode) -> String {
    match node {
        CSTNode::VarDec {
            id,
            var_dec,
            lt,
            rt,
            literal,
        } => {
            let mut var_name = String::new();
            if let Some(value) = id {
                var_name.push_str(value);
                var_name
            } else if let Some(value) = var_dec {
                let id_ = extract_vardec(value);
                let literal_value = literal
                    .clone()
                    .unwrap_or_default()
                    .parse::<u32>()
                    .expect("数组的长度必须是正整数");
                var_name.push_str(&id_);
                var_name.push_str(&lt.clone().unwrap_or_default());
                var_name.push_str(&literal_value.to_string());
                var_name.push_str(&rt.clone().unwrap_or_default());
                var_name
            } else {
                unreachable!()
            }
        }
        _ => unreachable!(),
    }
}
fn extract_param(node: &CSTNode) -> VarDec {
    match node {
        CSTNode::ParaDec { spec, var_dec } => {
            let var_type = extract_spec(spec);
            let var_name = extract_vardec(var_dec);
            VarDec {
                var_type,
                var_name,
                init: None,
            }
        }
        _ => unreachable!(),
    }
}
// TODO:
fn extract_fundec(node: &CSTNode) -> (String, Vec<VarDec>) {
    match node {
        CSTNode::FunDec { id, var_list, .. } => {
            set_current_func(id.clone());
            let mut params: Vec<VarDec> = vec![];
            if let Some(list) = var_list {
                collect_paradec(list, &mut params);
                (id.clone(), params)
            } else {
                (id.clone(), params)
            }
        }
        _ => unreachable!(),
    }
}
fn extract_struct_sepc(node: &CSTNode) -> StructType {
    match node {
        CSTNode::StructSpecifier { id, def_list, .. } => {
            let mut items: Vec<VarDec> = vec![];
            if let Some(list) = def_list {
                collect_defs(list, &mut items);
            }
            StructType {
                id: id.clone().unwrap_or_default(),
                field: items,
            }
        }
        _ => unreachable!(),
    }
}
fn extract_spec(spec: &CSTNode) -> Type {
    match spec {
        CSTNode::Specifier {
            specifier_type,
            struct_specifier,
        } => {
            if let Some(value) = specifier_type {
                Type::BaseType(value.clone())
            } else if let Some(struct_value) = struct_specifier {
                let ret = extract_struct_sepc(struct_value);
                Type::CustomizedType(ret)
            } else {
                unreachable!()
            }
        }
        _ => unreachable!(),
    }
}
// BUG:
fn collect_defs(node: &CSTNode, items: &mut Vec<VarDec>) {
    match node {
        CSTNode::DefList { def, def_list } => {
            let mut var_dec_list = extract_def(def);
            items.append(&mut var_dec_list);
            if let Some(list) = def_list {
                collect_defs(list, items);
            } else {
                // do nothing
            }
        }
        _ => {
            unreachable!();
        }
    }
}
// TODO:
// fn collect_local_decls(node: &CSTNode, items: &mut Vec<ASTNode>) {
//     todo!()
// }
// TODO:
fn collect_stmts(node: &CSTNode, items: &mut Vec<ASTNode>) {
    match node {
        CSTNode::StmtList { stmt, stmt_list } => {
            items.push(extract_stmt(stmt));
            if let Some(list) = stmt_list {
                collect_stmts(list, items);
            }
        }
        _ => unreachable!(),
    }
}
fn extract_stmt(node: &CSTNode) -> ASTNode {
    match node {
        CSTNode::Stmt {
            unmatched_stmt,
            matched_stmt,
        } => match (unmatched_stmt, matched_stmt) {
            (Some(stmt), None) => ASTNode::from_cst(stmt),
            (None, Some(stmt)) => ASTNode::from_cst(stmt),
            _ => {
                unreachable!()
            }
        },
        _ => unreachable!(),
    }
}
fn extract_def(node: &CSTNode) -> Vec<VarDec> {
    match node {
        CSTNode::Def { spec, dec_list, .. } => {
            let var_type = extract_spec(spec);
            collect_decs(var_type, dec_list)
        }
        _ => unreachable!(),
    }
}
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
            args.push(ASTNode::from_cst(expression));
            if let Some(next_arg) = arguments_tail {
                args.extend(colloct_arguments(next_arg));
            }
            args
        }
        CSTNode::ArgumentsTail { expression, .. } => {
            vec![ASTNode::from_cst(expression)]
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
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
                ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                }
            } else {
                let rhs = ASTNode::from_cst(logical_or);
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
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
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
                let ast_node = ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                };
                extract_logical_or(&ast_node, prime)
            } else {
                let rhs = ASTNode::from_cst(logical_and);
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
                ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                }
            }
        }
        _ => {
            unreachable!();
        }
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
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
                let ast_node = ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                };
                extract_logical_and(&ast_node, prime)
            } else {
                let rhs = ASTNode::from_cst(equality);
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
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
fn extract_equality(lhs: &ASTNode, node: &CSTNode) -> ASTNode {
    match node {
        CSTNode::EqualityPrime {
            op,
            comparison,
            equality_prime,
        } => {
            if let Some(prime) = equality_prime {
                let rhs = ASTNode::from_cst(comparison);
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
                let ast_node = ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                };
                extract_equality(&ast_node, prime)
            } else {
                let rhs = ASTNode::from_cst(comparison);
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
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
fn extract_comparison(lhs: &ASTNode, node: &CSTNode) -> ASTNode {
    match node {
        CSTNode::ComparisonPrime {
            op,
            term,
            comparison_prime,
        } => {
            if let Some(prime) = comparison_prime {
                let rhs = ASTNode::from_cst(term);
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
                let ast_node = ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                };
                extract_comparison(&ast_node, prime)
            } else {
                let rhs = ASTNode::from_cst(term);
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
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
fn extract_term(lhs: &ASTNode, node: &CSTNode) -> ASTNode {
    match node {
        CSTNode::TermPrime {
            op,
            factor,
            term_prime,
        } => {
            if let Some(prime) = term_prime {
                let rhs = ASTNode::from_cst(factor);
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
                let ast_node = ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                };
                extract_term(&ast_node, prime)
            } else {
                let rhs = ASTNode::from_cst(factor);
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
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
fn extract_factor(lhs: &ASTNode, node: &CSTNode) -> ASTNode {
    match node {
        CSTNode::FactorPrime {
            op,
            unary,
            factor_prime,
        } => {
            if let Some(prime) = factor_prime {
                let rhs = ASTNode::from_cst(unary);
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
                let ast_node = ASTNode::BinaryOp {
                    op: Some(op.clone()),
                    lhs: Box::new(lhs.clone()),
                    rhs: Some(Box::new(rhs)),
                };
                extract_factor(&ast_node, prime)
            } else {
                let rhs = ASTNode::from_cst(unary);
                if !check_types(lhs, &rhs) {
                    println!("lhs's types is different from rhs, {:?} {:?}", lhs, rhs);
                    unreachable!();
                }
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
