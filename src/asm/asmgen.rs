use super::AVAILABLE_REGS;
use crate::ir::irgen::IrType;
use std::collections::HashMap;

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

pub fn asm_gen(irs: &[IrType]) -> String {
    let mut asm = String::new();
    let mut var_offset: HashMap<String, i32> = HashMap::new();
    let mut reg_map: HashMap<String, String> = HashMap::new();
    let mut offset = 0;
    let mut label_count = 0;
    let mut in_func = false;
    let mut arg_count = 0;
    let mut param_count = 0;
    let mut stack_size = 0;

    // 1. _start入口
    asm += ".section .text\n";
    asm += ".globl _start\n";
    asm += "_start:\n";
    asm += "    li sp, 0x80009000\n";
    asm += "    li s0, 0\n";
    asm += "    jal main\n";
    asm += "    ebreak\n";

    for ir in irs {
        match ir.op.as_str() {
            "MOV" => {
                let rd = reg_map.get(&ir.rd).unwrap();
                if ir.src1.parse::<i32>().is_ok() {
                    // mov 立即数
                    asm += &format!("    li {}, {}\n", rd, ir.src1);
                } else if let Some(src) = reg_map.get(&ir.src1) {
                    if *rd != *src {
                        asm += &format!("    mv {}, {}\n", rd, src);
                    }
                }
            }
            "+" | "-" | "*" | "/" => {
                // src1
                if ir.src1.parse::<i32>().is_ok() {
                    asm += &format!("    li t0, {}\n", ir.src1);
                } else if let Some(src) = reg_map.get(&ir.src1) {
                    if src != "t0" {
                        asm += &format!("    mv t0, {}\n", src);
                    }
                }
                // src2
                if ir.src2.parse::<i32>().is_ok() {
                    asm += &format!("    li t1, {}\n", ir.src2);
                } else if let Some(src) = reg_map.get(&ir.src2) {
                    if src != "t1" {
                        asm += &format!("    mv t1, {}\n", src);
                    }
                }
                // 运算
                let opstr = match ir.op.as_str() {
                    "+" => "add",
                    "-" => "sub",
                    "*" => "mul",
                    "/" => "div",
                    _ => unreachable!(),
                };
                asm += &format!("    {} t2, t0, t1\n", opstr);
                if let Some(rd) = reg_map.get(&ir.rd) {
                    if rd != "t1" {
                        asm += &format!("    mv {}, t2\n", rd);
                    }
                }
            }
            "RET" => {
                if !ir.src1.is_empty() {
                    if ir.src1.parse::<i32>().is_ok() {
                        asm += &format!("    li a0, {}\n", ir.src1);
                    } else if let Some(src) = reg_map.get(&ir.src1) {
                        if src != "a0" {
                            asm += &format!("    mv a0, {}\n", src);
                        }
                    }
                }
                asm += "    addi sp, sp, ";
                asm += &format!("{}\n", stack_size);
                asm += "    ret\n";
                in_func = false;
            }
            "LABEL" => {
                let label = if !ir.src1.is_empty() {
                    ir.src1.clone()
                } else {
                    label_count += 1;
                    format!("L{}", label_count)
                };
                asm += &format!("{}:\n", label);
            }
            "FUNC" => {
                let start_idx = irs.iter().position(|s| s == ir).unwrap() + 1;
                let mut reg_iter = AVAILABLE_REGS.iter();
                for ir in &irs[start_idx..] {
                    if ir.op == "ENDFUNC" {
                        break;
                    }
                    if !ir.rd.is_empty() && !var_offset.contains_key(&ir.rd) {
                        offset += 4;
                        var_offset.insert(ir.rd.clone(), -offset); // sp负偏移
                        if let Some(&reg) = reg_iter.next() {
                            reg_map.insert(ir.rd.clone(), reg.to_string());
                        } else {
                            println!("寄存器溢出");
                        }
                    }
                    if !ir.src1.is_empty()
                        && !is_hexadecimal(&ir.src1)
                        && !is_decimal(&ir.src1)
                        && !var_offset.contains_key(&ir.src1)
                    {
                        offset += 4;
                        var_offset.insert(ir.src1.clone(), -offset); // sp负偏移
                        if let Some(&reg) = reg_iter.next() {
                            reg_map.insert(ir.src1.clone(), reg.to_string());
                        } else {
                            println!("寄存器溢出");
                        }
                    }
                    if !ir.src2.is_empty()
                        && !is_hexadecimal(&ir.src2)
                        && !is_decimal(&ir.src2)
                        && !var_offset.contains_key(&ir.src2)
                    {
                        offset += 4;
                        var_offset.insert(ir.src2.clone(), -offset); // sp负偏移
                        if let Some(&reg) = reg_iter.next() {
                            reg_map.insert(ir.src2.clone(), reg.to_string());
                        } else {
                            println!("寄存器溢出");
                        }
                    }
                }
                let label = if !ir.src1.is_empty() {
                    ir.src1.clone()
                } else {
                    format!("L{}", label_count)
                };
                stack_size = ((offset + 15) / 16) * 16;
                asm += &format!("{}:\n", label);
                asm += &format!("    addi sp, sp, -{}\n", stack_size);
                in_func = true;
                param_count = 0;
            }
            "JMP" => {
                asm += &format!("    j {}\n", ir.rd);
            }
            "JZ" => {
                // 跳转目标 rd，条件 src1
                if ir.src1.parse::<i32>().is_ok() {
                    asm += &format!("    li t0, {}\n", ir.src1);
                } else if let Some(src) = reg_map.get(&ir.src1) {
                    asm += &format!("    mv t0, {}\n", src);
                }
                asm += &format!("    beqz t0, {}\n", ir.rd);
            }
            "JNZ" => {
                if ir.src1.parse::<i32>().is_ok() {
                    asm += &format!("    li t0, {}\n", ir.src1);
                } else if let Some(src) = reg_map.get(&ir.src1) {
                    asm += &format!("    mv t0, {}\n", src);
                }
                asm += &format!("    bnez t0, {}\n", ir.rd);
            }
            "ARG" => {
                if ir.src1.parse::<i32>().is_ok() {
                    asm += &format!("    li a{}, {}\n", arg_count, ir.src1);
                } else if let Some(src) = reg_map.get(&ir.src1) {
                    asm += &format!("    mv a{}, {}\n", arg_count, src);
                }
                arg_count += 1;
            }
            "CALL" => {
                let func_name = &ir.src1;
                asm += &format!("    call {}\n", func_name);
                // 如果有rd，保存返回值
                if !ir.rd.is_empty() {
                    if let Some(rd) = reg_map.get(&ir.rd) {
                        asm += &format!("    mv {}, a0\n", rd);
                    }
                }
                arg_count = 0;
            }
            "ENDFUNC" => {
                if in_func {
                    asm += &format!("     addi sp, sp, {}\n", stack_size);
                    asm += "    ret\n";
                    in_func = false;
                }
                var_offset.clear();
                reg_map.clear();
            }
            "PARAM" => {
                let reg_name = reg_map.get(&ir.src1).unwrap();
                let param_reg = format!("a{}", param_count);
                if param_reg != *reg_name {
                    asm += &format!("    mv {}, {}\n", reg_name, param_reg);
                }
                param_count += 1;
            }
            "<" => {
                let src_reg = reg_map.get(&ir.src1).unwrap();
                let rd_reg = reg_map.get(&ir.rd).unwrap();
                if ir.src2.parse::<i32>().is_ok() {
                    asm += &format!("    slti {}, {}, {}\n", rd_reg, src_reg, ir.src2);
                } else {
                    let src2_reg = reg_map.get(&ir.src2).unwrap();
                    asm += &format!("    slt {}, {}, {}\n", rd_reg, src_reg, src2_reg);
                }
            }
            "=" => {
                let src_reg = reg_map.get(&ir.src1).unwrap();
                let rd_reg = reg_map.get(&ir.rd).unwrap();
                asm += &format!("    mv {}, {}\n", rd_reg, src_reg);
            }
            // 扩展
            _ => {}
        }
    }
    // 若没RET，补一个返回
    if !asm.contains("ret") {
        asm += "     addi sp, sp, ";
        asm += &format!("{}\n", stack_size);
        asm += "    ret\n";
    }

    asm
}
