use super::asmgen::asm_gen;
use crate::ir::irgen::IrType;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn change_extension(filename: &String) -> String {
    let path = Path::new(filename);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    format!("{}.{}", stem, String::from("asm"))
}

pub fn write_asm(ir: &[IrType], filename: &String) {
    let asm = asm_gen(ir);

    let target_file_name = change_extension(filename);
    let mut f = File::create(target_file_name).unwrap();
    f.write_all(asm.as_bytes()).unwrap();
}
