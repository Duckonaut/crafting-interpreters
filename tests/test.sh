#!/bin/bash

set -e

usage() {
    echo "Usage: $0 -e <executable> -d <directory> -t <test>"
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

TESTS=$(find $DIRECTORY -name "*.hez")
TEST_COUNT=$(echo "$TESTS" | wc -l)
TESTS_PASSED=0

printf "Running tests in $DIRECTORY\n"

for test in $TESTS; do
    printf "Running $test... "

    # Run the test
    output=$($EXECUTABLE "$test")

    # Check the output
    expected=$(cat $(echo $test | sed 's/\.hez$/\.out/'))
    if [ "$output" = "$expected" ]; then
        printf "OK${NC}\n"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        printf "${RED}FAIL${NC}\n"
        printf "Output diff:\n"
        diff <(echo "$output") <(echo "$expected")
    fi
done

if [ $TESTS_PASSED -eq $TEST_COUNT ]; then
    printf "${GREEN}All tests passed${NC}\n"
else
    printf "${RED}$TESTS_PASSED/$TEST_COUNT tests passed${NC}\n"
fi
