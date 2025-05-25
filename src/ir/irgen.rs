use crate::ast::types::ASTNode;

use super::optimal::optimal;

#[derive(Debug, Clone, PartialEq)]
pub struct IrType {
    pub op: String,
    pub src1: String,
    pub src2: String,
    pub rd: String,
}

pub fn ir_gen(ast: &Vec<ASTNode>) -> Vec<IrType> {
    let mut code = Vec::new();
    let mut temp_id = 0;
    let mut label_id = 0;
    for ast_node in ast {
        ir_gen_recursive(ast_node, &mut code, &mut temp_id, &mut label_id);
    }
    // code
    optimal(&code)
}

pub fn ir_gen_recursive(
    node: &ASTNode,
    code: &mut Vec<IrType>,
    temp_id: &mut usize,
    label_id: &mut usize,
) -> Option<String> {
    // 临时变量生成器
    fn new_temp(temp_id: &mut usize) -> String {
        let t = format!("t{}", *temp_id);
        *temp_id += 1;
        t
    }
    // 标签生成器
    fn new_label(label_id: &mut usize) -> String {
        let l = format!("L{}", *label_id);
        *label_id += 1;
        l
    }

    match node {
        ASTNode::Program { items } => {
            for item in items {
                ir_gen_recursive(item, code, temp_id, label_id);
            }
            None
        }
        ASTNode::FuncDef {
            name, params, body, ..
        } => {
            code.push(IrType {
                op: "FUNC".to_string(),
                src1: name.clone(),
                src2: "".to_string(),
                rd: "".to_string(),
            });
            for param in params {
                code.push(IrType {
                    op: "PARAM".to_string(),
                    src1: param.var_name.clone(),
                    src2: "".to_string(),
                    rd: "".to_string(),
                });
            }
            ir_gen_recursive(body, code, temp_id, label_id);
            code.push(IrType {
                op: "ENDFUNC".to_string(),
                src1: name.clone(),
                src2: "".to_string(),
                rd: "".to_string(),
            });
            None
        }
        ASTNode::Block { stmts } => {
            for stmt in stmts {
                ir_gen_recursive(stmt, code, temp_id, label_id);
            }
            None
        }
        ASTNode::VarDecl(decls) => {
            for decl in decls {
                if let Some(init) = &decl.init {
                    let src1 = ir_gen_recursive(init, code, temp_id, label_id).unwrap_or_default();
                    code.push(IrType {
                        op: "MOV".to_string(),
                        src1,
                        src2: "".to_string(),
                        rd: decl.var_name.clone(),
                    });
                }
            }
            None
        }
        ASTNode::If {
            cond,
            then_br,
            else_br,
        } => {
            let cond_temp = ir_gen_recursive(cond, code, temp_id, label_id).unwrap_or_default();
            let else_label = new_label(label_id);
            let end_label = new_label(label_id);
            code.push(IrType {
                op: "JZ".to_string(),
                src1: cond_temp,
                src2: "".to_string(),
                rd: else_label.clone(),
            });
            ir_gen_recursive(then_br, code, temp_id, label_id);
            code.push(IrType {
                op: "JMP".to_string(),
                src1: "".to_string(),
                src2: "".to_string(),
                rd: end_label.clone(),
            });
            code.push(IrType {
                op: "LABEL".to_string(),
                src1: else_label.clone(),
                src2: "".to_string(),
                rd: "".to_string(),
            });
            if let Some(else_br) = else_br {
                ir_gen_recursive(else_br, code, temp_id, label_id);
            }
            code.push(IrType {
                op: "LABEL".to_string(),
                src1: end_label.clone(),
                src2: "".to_string(),
                rd: "".to_string(),
            });
            None
        }
        ASTNode::While { cond, body } => {
            let start_label = new_label(label_id);
            let cond_label = new_label(label_id);
            let end_label = new_label(label_id);
            code.push(IrType {
                op: "JMP".to_string(),
                src1: "".to_string(),
                src2: "".to_string(),
                rd: cond_label.clone(),
            });
            code.push(IrType {
                op: "LABEL".to_string(),
                src1: start_label.clone(),
                src2: "".to_string(),
                rd: "".to_string(),
            });
            ir_gen_recursive(body, code, temp_id, label_id);
            code.push(IrType {
                op: "LABEL".to_string(),
                src1: cond_label.clone(),
                src2: "".to_string(),
                rd: "".to_string(),
            });
            let cond_temp = ir_gen_recursive(cond, code, temp_id, label_id).unwrap_or_default();
            code.push(IrType {
                op: "JNZ".to_string(),
                src1: cond_temp,
                src2: "".to_string(),
                rd: start_label.clone(),
            });
            code.push(IrType {
                op: "LABEL".to_string(),
                src1: end_label.clone(),
                src2: "".to_string(),
                rd: "".to_string(),
            });
            None
        }
        ASTNode::Return { expr } => {
            if let Some(e) = expr {
                let val = ir_gen_recursive(e, code, temp_id, label_id).unwrap_or_default();
                code.push(IrType {
                    op: "RET".to_string(),
                    src1: val,
                    src2: "".to_string(),
                    rd: "".to_string(),
                });
            } else {
                code.push(IrType {
                    op: "RET".to_string(),
                    src1: "".to_string(),
                    src2: "".to_string(),
                    rd: "".to_string(),
                });
            }
            None
        }
        ASTNode::BinaryOp { op, lhs, rhs } => {
            let left = ir_gen_recursive(lhs, code, temp_id, label_id).unwrap_or_default();
            let right = ir_gen_recursive(rhs.as_ref().unwrap(), code, temp_id, label_id)
                .unwrap_or_default();
            if op.clone().unwrap_or_default() == "=" {
                let ir_node = IrType {
                    op: op.clone().unwrap_or_default(),
                    src1: right,
                    src2: "".to_string(),
                    rd: left.clone(),
                };
                code.push(ir_node);
                Some(left)
            } else {
                let temp = new_temp(temp_id);
                let ir_node = IrType {
                    op: op.clone().unwrap_or_default(),
                    src1: left,
                    src2: right,
                    rd: temp.clone(),
                };
                code.push(ir_node);
                Some(temp)
            }
        }
        ASTNode::UnaryOp { op, expr } => {
            let val = ir_gen_recursive(expr, code, temp_id, label_id).unwrap_or_default();
            let temp = new_temp(temp_id);
            code.push(IrType {
                op: op.clone(),
                src1: val,
                src2: "".to_string(),
                rd: temp.clone(),
            });
            Some(temp)
        }
        ASTNode::Call { func, args } => {
            let mut arg_vars = Vec::new();
            for arg in args {
                let v = ir_gen_recursive(arg, code, temp_id, label_id).unwrap_or_default();
                arg_vars.push(v);
            }
            for v in &arg_vars {
                code.push(IrType {
                    op: "ARG".to_string(),
                    src1: v.clone(),
                    src2: "".to_string(),
                    rd: "".to_string(),
                });
            }
            let temp = new_temp(temp_id);
            code.push(IrType {
                op: "CALL".to_string(),
                src1: func.clone(),
                src2: arg_vars.len().to_string(),
                rd: temp.clone(),
            });
            Some(temp)
        }
        ASTNode::Literal(tok) => Some(tok.value.clone()),
        ASTNode::Ident(tok) => Some(tok.value.clone()),
        _ => None,
    }
}
