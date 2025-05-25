#!/bin/bash

INPUT_FILE="$1"
LOG_FILE="./build.log"

# 清空日志文件
>"$LOG_FILE"

# 检查输入参数
if [ -z "$INPUT_FILE" ]; then
  echo "Error: No input file specified" | tee -a "$LOG_FILE"
  exit 1
fi

# 检查输入文件是否存在
if [ ! -f "$INPUT_FILE" ]; then
  echo "Error: Input file $INPUT_FILE does not exist" | tee -a "$LOG_FILE"
  exit 1
fi

BASENAME="${INPUT_FILE%.*}"
TMP_FILE="${BASENAME}.o"
OUTPUT_FILE="${BASENAME}.elf"

# 创建 build 目录
mkdir -p build

# 汇编
riscv64-linux-gnu-as -march=rv32im -mabi=ilp32 -o "./build/$TMP_FILE" "$INPUT_FILE" >>"$LOG_FILE" 2>&1
if [ $? -ne 0 ]; then
  echo "Assembly failed. See $LOG_FILE for details." | tee -a "$LOG_FILE"
  exit 1
fi

# 链接
riscv64-linux-gnu-gcc -nostdlib -march=rv32im -mabi=ilp32 -T ./tools/linker.ld "./build/$TMP_FILE" -o "./build/$OUTPUT_FILE" >>"$LOG_FILE" 2>&1
if [ $? -ne 0 ]; then
  echo "Linking failed. See $LOG_FILE for details." | tee -a "$LOG_FILE"
  exit 1
fi

echo "$OUTPUT_FILE successfully generated" | tee -a "$LOG_FILE"
