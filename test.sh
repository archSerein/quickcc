#!/bin/bash

cargo build >/dev/null 2>&1
# Check if input file is provided
if [ $# -eq 0 ]; then
  echo "Usage: $0 <input_file>"
  echo "Example: $0 test1"
  echo "This will process ./test/test1.c and output to a log file"
  exit 1
fi

INPUT_FILE="$1"
TEST_FILE="./test/${INPUT_FILE}.c"
LOG_FILE="./log/log_$(date +%Y%m%d_%H%M%S).log"

# Check if input file exists
if [ ! -f "$TEST_FILE" ]; then
  echo "Error: Input file $TEST_FILE does not exist"
  exit 1
fi

# Print input file being processed
echo "Processing $TEST_FILE"

# Run quickcc and redirect output (stdout and stderr) to log
./target/debug/quickcc "$TEST_FILE" >"$LOG_FILE" 2>&1

# Check if quickcc ran successfully
if [ $? -eq 0 ]; then
  echo "Compilation successful. Output written to $LOG_FILE"
else
  echo "Compilation failed. Check $LOG_FILE for details"
  exit 1
fi
