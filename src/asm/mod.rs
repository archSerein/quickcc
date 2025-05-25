pub mod asmgen;
pub mod write_asm;

const AVAILABLE_REGS: [&str; 20] = [
    "t0", "t3", "t4", "t5", "t6", "a5", "a6", "a7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
    "s8", "s9", "s10", "s11",
];
