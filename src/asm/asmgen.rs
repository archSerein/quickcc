use crate::ir::irgen::IrType;
use std::collections::HashMap;

fn is_hexadecimal(s: &str) -> bool {
    if let Some(rest) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        return u64::from_str_radix(rest, 16).is_ok();
    }
    false
}

fn is_decimal(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}

pub fn asm_gen(irs: &[IrType]) -> String {
    let mut asm = String::new();
    let mut functions = Vec::new();
    let mut i = 0;

    while i < irs.len() {
        if irs[i].op == "FUNC" {
            let func_name = irs[i].src1.clone();
            let mut params = Vec::new();
            let mut locals = Vec::new();
            let mut body = Vec::new();
            i += 1;

            while i < irs.len() && irs[i].op != "ENDFUNC" {
                let ir = irs[i].clone();
                body.push(ir.clone());
                if ir.op == "PARAM" {
                    if !params.contains(&ir.src1) {
                        params.push(ir.src1.clone());
                    }
                } else {
                    for var in [&ir.rd, &ir.src1, &ir.src2] {
                        if !var.is_empty()
                            && !is_decimal(var)
                            && !is_hexadecimal(var)
                            && !params.contains(var)
                            && !locals.contains(var)
                        {
                            locals.push(var.clone());
                        }
                    }
                }
                i += 1;
            }
            functions.push((func_name, params, locals, body));
        }
        i += 1;
    }

    asm += ".section .text\n";
    asm += ".globl _start\n";
    asm += "_start:\n";
    asm += "    li sp, 0x80009000\n";
    asm += "    call main\n";
    asm += "    ebreak\n";

    for (func_name, params, locals, body) in functions {
        let mut reg_map: HashMap<String, String> = HashMap::new();
        let mut stack_offset = 0;
        let mut reg_idx = 0;
        let offset = (locals.len() * 4 + 15) / 16 * 16;
        let mut off = offset - 4;
        let all_regs = [
            "t2", "t3", "t4", "t5", "t6", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8",
            "s9", "s10", "s11",
        ];

        asm += &format!("{}:\n", func_name);
        asm += &format!("    addi sp, sp, -{}\n", offset);
        asm += &format!("    sw ra, {}(sp)\n", off);

        for (i, param) in params.iter().enumerate() {
            let preg = format!("a{}", i);
            let reg = all_regs[reg_idx % all_regs.len()];
            reg_map.insert(param.clone(), reg.to_string());
            asm += &format!("    mv {}, {}\n", reg, preg);
            reg_idx += 1;
        }

        for local in locals.clone() {
            if !reg_map.contains_key(&local) {
                let reg = all_regs[reg_idx % all_regs.len()];
                reg_map.insert(local.clone(), reg.to_string());
                reg_idx += 1;
            }
        }

        for ir in body {
            match ir.op.as_str() {
                "MOV" => {
                    let dst = reg_map.get(&ir.rd).unwrap();
                    if is_decimal(&ir.src1) {
                        asm += &format!("    li {}, {}\n", dst, ir.src1);
                    } else {
                        let src = reg_map.get(&ir.src1).unwrap();
                        asm += &format!("    mv {}, {}\n", dst, src);
                    }
                }
                "+" | "-" | "*" | "/" => {
                    let op = match ir.op.as_str() {
                        "+" => "add",
                        "-" => "sub",
                        "*" => "mul",
                        "/" => "div",
                        _ => unreachable!(),
                    };
                    let dst = reg_map.get(&ir.rd).unwrap();
                    let src1 = if is_decimal(&ir.src1) {
                        asm += "    li t0, ";
                        asm += &ir.src1;
                        asm += "\n";
                        "t0"
                    } else {
                        reg_map.get(&ir.src1).unwrap()
                    };
                    let src2 = if is_decimal(&ir.src2) {
                        asm += "    li t1, ";
                        asm += &ir.src2;
                        asm += "\n";
                        "t1"
                    } else {
                        reg_map.get(&ir.src2).unwrap()
                    };
                    asm += &format!("    {} {}, {}, {}\n", op, dst, src1, src2);
                }
                "RET" => {
                    if !ir.src1.is_empty() {
                        if is_decimal(&ir.src1) {
                            asm += &format!("    li a0, {}\n", ir.src1);
                        } else {
                            let r = reg_map.get(&ir.src1).unwrap();
                            asm += &format!("    mv a0, {}\n", r);
                        };
                    }
                    asm += &format!("    lw ra, {}(sp)\n", offset - 4);
                    asm += &format!("    addi sp, sp, {}\n", offset);
                    asm += "    ret\n";
                }
                "CALL" => {
                    for local in locals.clone() {
                        let reg_name = reg_map.get(&local).unwrap();
                        off -= 4;
                        asm += &format!("    sw {}, {}(sp)\n", reg_name, off);
                    }
                    asm += &format!("    call {}\n", ir.src1);
                    for local in locals.clone().iter().rev() {
                        let reg_name = reg_map.get(local).unwrap();
                        asm += &format!("    lw {}, {}(sp)\n", reg_name, off);
                        off += 4;
                    }
                    if !ir.rd.is_empty() {
                        let dst = reg_map.get(&ir.rd).unwrap();
                        asm += &format!("    mv {}, a0\n", dst);
                    }
                }
                "ARG" => {
                    let idx = stack_offset;
                    let src = reg_map.get(&ir.src1).unwrap();
                    asm += &format!("    mv a{}, {}\n", idx, src);
                    stack_offset += 1;
                }
                "JMP" => {
                    asm += &format!("    j {}\n", ir.rd);
                }
                "JNZ" => {
                    let cond = reg_map.get(&ir.src1).unwrap();
                    asm += &format!("    bnez {}, {}\n", cond, ir.rd);
                }
                "LABEL" => {
                    asm += &format!("{}:\n", ir.src1);
                }
                "<" => {
                    let dst = reg_map.get(&ir.rd).unwrap();
                    let left = reg_map.get(&ir.src1).unwrap();
                    if is_decimal(&ir.src2) {
                        asm += &format!("    slti {}, {}, {}\n", dst, left, ir.src2);
                    } else {
                        let right = reg_map.get(&ir.src2).unwrap();
                        asm += &format!("    slt {}, {}, {}\n", dst, left, right);
                    }
                }
                "=" => {
                    let src = reg_map.get(&ir.src1).unwrap();
                    let rd = reg_map.get(&ir.rd).unwrap();
                    asm += &format!("    mv {}, {}\n", rd, src);
                }
                _ => {}
            }
        }
    }
    asm
}
