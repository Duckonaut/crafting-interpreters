#!/bin/bash

set -e

# Color vars
SUCCESS_COLOR='\033[32;1m'
WARNING_COLOR='\033[33m'
ERROR_COLOR='\033[31;1m'
MESSAGE_COLOR='\033[94;1m'
NC='\033[0m'

usage() {
    echo "Usage: $0 -e <executable> -d <directory> -t <test-pattern>"
    exit 1
}

EXECUTABLE="../hezen/target/release/hezen run"
DIRECTORY="."
TEST='*'

while getopts "e:d:t:" opt; do
    case $opt in
        e)
            EXECUTABLE=$OPTARG
            ;;
        d)
            DIRECTORY=$OPTARG
            ;;
        t)
            TEST=$OPTARG
            ;;
        *)
            usage
            ;;
    esac
done

# Gather the script files with their expected outputs

printf "Directory: $DIRECTORY\n"
TESTS=$(find $DIRECTORY -name "$TEST.hez")
TEST_COUNT=$(echo "$TESTS" | wc -l)
TESTS_PASSED=0

if [ $TEST_COUNT -eq 0 ] || [ -z "$TESTS" ]; then
    printf "${ERROR_COLOR}No tests found!${NC}\n${MESSAGE_COLOR}If you are sure there are tests, try using a different test name pattern.${NC}\n"
    exit 1
fi

printf "Discovered ${MESSAGE_COLOR}${TEST_COUNT}${NC} tests\n"

for test in $TESTS; do
    printf "Running ${MESSAGE_COLOR}$test${NC}... "

    # Run the test
    output=$($EXECUTABLE "$test")

    # Check the output
    expected=$(cat $(echo $test | sed 's/\.hez$/\.out/'))
    if [ "$output" = "$expected" ]; then
        printf "${SUCCESS_COLOR}OK${NC}\n"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        printf "${ERROR_COLOR}FAIL${NC}\n"
        printf "Output diff:\n"
        diff --color=always -u <(echo "$expected") <(echo "$output")
    fi
done

if [ $TESTS_PASSED -eq $TEST_COUNT ]; then
    printf "${SUCCESS_COLOR}All tests passed${NC}\n"
else
    printf "${ERROR_COLOR}$TESTS_PASSED/$TEST_COUNT tests passed${NC}\n"
fi
