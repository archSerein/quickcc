use super::irgen::IrType;

pub fn const_fold_and_propagate(ir_list: &[IrType]) -> Vec<IrType> {
    use std::collections::HashMap;
    let mut consts: HashMap<String, String> = HashMap::new();
    let mut result = Vec::new();

    for ir in ir_list.iter() {
        let op = ir.op.as_str();
        // 先常量传播
        let src1 = if consts.contains_key(&ir.src1) {
            consts[&ir.src1].clone()
        } else {
            ir.src1.clone()
        };
        let src2 = if consts.contains_key(&ir.src2) {
            consts[&ir.src2].clone()
        } else {
            ir.src2.clone()
        };

        // 遇到跳转或标签清空常量表
        if matches!(op, "JMP" | "JZ" | "JNZ" | "LABEL" | "CALL") {
            consts.clear();
            result.push(IrType {
                op: ir.op.clone(),
                src1,
                src2,
                rd: ir.rd.clone(),
            });
            continue;
        }

        // 常量合并
        if (op == "+" || op == "-" || op == "*" || op == "/")
            && src1.parse::<i64>().is_ok()
            && src2.parse::<i64>().is_ok()
        {
            let a = src1.parse::<i64>().unwrap();
            let b = src2.parse::<i64>().unwrap();
            let val = match op {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => unreachable!(),
            };
            // 常量传播
            consts.insert(ir.rd.clone(), val.to_string());
            result.push(IrType {
                op: "MOV".to_string(),
                src1: val.to_string(),
                src2: "".to_string(),
                rd: ir.rd.clone(),
            });
        } else if op == "MOV" {
            // 只在右值为常量时才传播
            if src1.parse::<i64>().is_ok() {
                consts.insert(ir.rd.clone(), src1.clone());
                result.push(IrType {
                    op: ir.op.clone(),
                    src1,
                    src2: "".to_string(),
                    rd: ir.rd.clone(),
                });
            } else {
                // 右值不是常量，杀死常量
                consts.remove(&ir.rd);
                result.push(IrType {
                    op: ir.op.clone(),
                    src1,
                    src2: "".to_string(),
                    rd: ir.rd.clone(),
                });
            }
        } else {
            consts.remove(&ir.rd);
            result.push(IrType {
                op: ir.op.clone(),
                src1,
                src2,
                rd: ir.rd.clone(),
            });
        }
    }
    result
}

fn is_hexadecimal(s: &str) -> bool {
    // 必须以0x或0X开头，并且后面是合法的十六进制字符
    if let Some(rest) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        return u64::from_str_radix(rest, 16).is_ok();
    }
    false
}

fn is_decimal(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}

fn dead_code_elimination(ir_list: Vec<IrType>) -> Vec<IrType> {
    use std::collections::HashSet;
    // 统计所有被用到的变量
    let mut used = HashSet::new();
    for ir in &ir_list {
        if !ir.src1.is_empty() && !is_decimal(&ir.src1) && !is_hexadecimal(&ir.src1) {
            used.insert(ir.src1.clone());
        }
        if !ir.src2.is_empty() && !is_decimal(&ir.src2) && !is_hexadecimal(&ir.src2) {
            used.insert(ir.src2.clone());
        }
    }
    // 仅保留rd被用到或没有rd的四元式
    ir_list
        .into_iter()
        .filter(|ir| ir.rd.is_empty() || used.contains(&ir.rd) || !ir.rd.starts_with('t'))
        .collect()
}

fn common_subexpression_elimination(ir_list: Vec<IrType>) -> Vec<IrType> {
    use std::collections::HashMap;
    let mut expr_map: HashMap<(String, String, String), String> = HashMap::new();
    let mut result = Vec::new();

    for ir in ir_list {
        let key = (ir.op.clone(), ir.src1.clone(), ir.src2.clone());
        if ["+", "-", "*", "/"].contains(&ir.op.as_str()) {
            if let Some(prev_rd) = expr_map.get(&key) {
                // 用上次的结果
                result.push(IrType {
                    op: "MOV".to_string(),
                    src1: prev_rd.clone(),
                    src2: "".to_string(),
                    rd: ir.rd,
                });
            } else {
                expr_map.insert(key, ir.rd.clone());
                result.push(ir);
            }
        } else {
            // 非表达式则直接加
            result.push(ir);
        }
    }
    result
}

fn loop_unrolling(ir_list: Vec<IrType>) -> Vec<IrType> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < ir_list.len() {
        let ir = &ir_list[i];
        if ir.op == "LABEL" && i + 4 < ir_list.len() {
            // 检查是否while模式，且循环次数为常量
            let cond_ir = &ir_list[i + 2];
            let jnz_ir = &ir_list[i + 3];
            if cond_ir.op == "<" && cond_ir.src2.parse::<usize>().is_ok() && jnz_ir.op == "JNZ" {
                let n = cond_ir.src2.parse::<usize>().unwrap();
                let loop_body = ir_list[i + 1].clone();
                // 展开n次
                for _ in 0..n {
                    result.push(loop_body.clone());
                }
                i += 5; // 跳过整个循环结构
                continue;
            }
        }
        result.push(ir.clone());
        i += 1;
    }
    result
}

pub fn optimal(ir: &[IrType]) -> Vec<IrType> {
    let ir_folded = const_fold_and_propagate(ir);
    let ir_dead_elimination = dead_code_elimination(ir_folded);
    let ir_subexpression_elimination = common_subexpression_elimination(ir_dead_elimination);
    loop_unrolling(ir_subexpression_elimination)
}
