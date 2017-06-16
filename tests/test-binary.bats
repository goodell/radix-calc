#!/usr/bin/env bats

# this is a basic bats test script (https://github.com/sstephenson/bats)
# the goal here is to check for output formatting errors and basic option
# handling bugs, not to rigorously test the grammar parser via the command line

@test "decimal addition" {
    result="$(radix-calc 5+5)"
    [[ "$result" = "10" ]]
}

@test "hexadecimal output" {
    result="$(radix-calc --hex '5*5')"
    [[ "$result" = "0x19" ]]
}

@test "octal output" {
    result="$(radix-calc --oct '5*5')"
    [[ "$result" = "0o31" ]]
}

@test "binary output" {
    result="$(radix-calc --bin '5*5')"
    [[ "$result" = "0b11001" ]]
}

@test "--all output" {
    result="$(radix-calc --all '5*5')"
    expected="25
0x19
0o31
0b11001"
    [[ "$result" = "$expected" ]]
}

@test "--alfred2 output" {
    result="$(radix-calc --alfred2 '5*5')"
    expected=$(cat tests/expected/alfred2-test0.xml)
    [[ "$result" = "$expected" ]]

    result="$(radix-calc --alfred2 '1234 << 33')"
    expected=$(cat tests/expected/alfred2-test1.xml)
    [[ "$result" = "$expected" ]]
}

valid_usage() {
    case "$1" in
        (*"Usage: radix-calc"*"Options:"*)
            ;;
        (*)
            false "invalid Usage output"
            ;;
    esac
}

# This covers a regression where the space was missing between "-h" and "-help"
# in the "Options:" section, causing "-h" to be treated as an (invalid)
# arithmetic expression.
@test "-h output" {
    result="$(radix-calc -h)"
    valid_usage "$result"

    # full --help should give the same results
    result="$(radix-calc --help)"
    valid_usage "$result"
}

# vim: set ft=sh :
