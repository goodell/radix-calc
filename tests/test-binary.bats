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
    expected="<?xml version=\"1.0\"?>
<items>
  <item arg=\"25\" valid=\"YES\" autocomplete=\"25\" type=\"default\">
    <title>25</title>
    <subtitle>(decimal)</subtitle>
    <icon>dec.png</icon>
  </item>
  <item arg=\"0x19\" valid=\"YES\" autocomplete=\"0x19\" type=\"default\">
    <title>0x19</title>
    <subtitle>(hexadecimal)</subtitle>
    <icon>hex.png</icon>
  </item>
  <item arg=\"0o31\" valid=\"YES\" autocomplete=\"0o31\" type=\"default\">
    <title>0o31</title>
    <subtitle>(octal)</subtitle>
    <icon>oct.png</icon>
  </item>
  <item arg=\"0b11001\" valid=\"YES\" autocomplete=\"0b11001\" type=\"default\">
    <title>0b11001</title>
    <subtitle>(binary)</subtitle>
    <icon>bin.png</icon>
  </item>
</items>"
    [[ "$result" = "$expected" ]]
}

# This covers a regression where the space was missing between "-h" and "-help"
# in the "Options:" section
#
# In the future it may be desirable to match the Usage message in a fuzzy
# manner so that we don't have to update this test every single time we tweak
# the Usage output.
@test "-h output" {
    result="$(radix-calc -h)"
    expected="A programmer's calculator supporting multiple radixes.

Usage: radix-calc [--alfred2] [--all|--bin|--hex|--oct] [--] <expr>...
       radix-calc (-h | --help)

Options:
    -h, --help   Show this screen.
    --alfred2    Emit Alfred2-style workflow XML.
    --all        Format the result in decimal, hex, octal, and binary
    --bin        Format the result in binary (e.g., 0b0110)
    --hex        Format the result in hexadecimal (e.g., 0xcafe)
    --oct        Format the result in Rust-style octal (e.g., 0o755)"
    [[ "$result" = "$expected" ]]

    # full --help should give the same results
    result="$(radix-calc --help)"
    [[ "$result" = "$expected" ]]
}

# vim: set ft=sh :
