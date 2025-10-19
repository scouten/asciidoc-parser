#!/usr/bin/env python3
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(__file__)))
SRC_DIR = os.path.join(ROOT, "src")

TEST_FN_PATTERN = re.compile(r"^\s*#\[test\]\s*$")
FN_DEF_PATTERN = re.compile(r"^\s*fn\s+([A-Za-z0-9_]+)")
TEST_MOD_PATTERN = re.compile(r"^\s*#\[cfg\(test\)\]\s*$")
MOD_DEF_PATTERN = re.compile(r"^\s*mod\s+tests\s*\{")

def find_test_functions(root=SRC_DIR):
    tests = set()
    for dirpath, _, filenames in os.walk(root):
        for fname in filenames:
            if not fname.endswith(".rs"):
                continue
            path = os.path.join(dirpath, fname)
            with open(path, "r", encoding="utf-8") as f:
                lines = f.readlines()
            for i, line in enumerate(lines[:-1]):
                if TEST_FN_PATTERN.match(line):
                    m = FN_DEF_PATTERN.match(lines[i + 1])
                    if m:
                        tests.add(m.group(1))
    return tests

def find_test_module_ranges(root=SRC_DIR):
    mod_ranges = {}
    for dirpath, _, filenames in os.walk(root):
        for fname in filenames:
            if not fname.endswith(".rs"):
                continue
            path = os.path.join(dirpath, fname)
            with open(path, "r", encoding="utf-8") as f:
                lines = f.readlines()

            i = 0
            ranges = []
            while i < len(lines) - 1:
                if TEST_MOD_PATTERN.match(lines[i]) and MOD_DEF_PATTERN.match(lines[i + 1]):
                    start = i + 2
                    depth = 1
                    j = start
                    while j < len(lines) and depth > 0:
                        depth += lines[j].count("{")
                        depth -= lines[j].count("}")
                        j += 1
                    end = j
                    ranges.append((start + 1, end))
                    i = j
                else:
                    i += 1
            if ranges:
                mod_ranges[path] = ranges
    return mod_ranges

def in_test_module(file_path, line_num, test_mod_ranges):
    if file_path not in test_mod_ranges:
        return False
    for start, end in test_mod_ranges[file_path]:
        if start <= line_num <= end:
            return True
    return False

def filter_lcov(infile, outfile, test_funcs, test_mod_ranges):
    skip_block = False
    current_file = None
    excluded_fns = 0
    excluded_lines = 0
    excluded_files = set()

    for line in infile:
        if line.startswith("SF:"):
            current_file = line.strip().split("SF:")[1]
            skip_block = "/tests/" in current_file or current_file.endswith("/tests.rs")
            if skip_block:
                excluded_files.add(current_file)
        elif line.startswith("end_of_record"):
            skip_block = False

        if skip_block:
            continue

        if line.startswith(("FN:", "FNDA:")):
            fn_name = line.strip().split(",")[-1]
            if fn_name in test_funcs:
                excluded_fns += 1
                continue
            m = re.match(r"FN:(\d+),", line)
            if m and in_test_module(current_file, int(m.group(1)), test_mod_ranges):
                excluded_fns += 1
                continue

        if line.startswith("DA:"):
            parts = line.strip().split(",")
            try:
                line_num = int(parts[0][3:])
                if in_test_module(current_file, line_num, test_mod_ranges):
                    excluded_lines += 1
                    continue
            except ValueError:
                pass

        outfile.write(line)

    return excluded_files, excluded_fns, excluded_lines

def main():
    test_funcs = find_test_functions()
    test_mod_ranges = find_test_module_ranges()
    print(f"🔍 Found {len(test_funcs)} #[test] functions", file=sys.stderr)
    print(f"🔍 Found {sum(len(v) for v in test_mod_ranges.values())} test modules", file=sys.stderr)

    with open("lcov.info", "r", encoding="utf-8") as infile,              open("lcov.filtered.info", "w", encoding="utf-8") as outfile:
        excluded_files, excluded_fns, excluded_lines = filter_lcov(infile, outfile, test_funcs, test_mod_ranges)

    print(f"🧹 Excluded {len(excluded_files)} test files", file=sys.stderr)
    print(f"🧹 Excluded {excluded_fns} test functions", file=sys.stderr)
    print(f"🧹 Excluded {excluded_lines} test lines", file=sys.stderr)
    print("✅ Wrote filtered LCOV to lcov.filtered.info", file=sys.stderr)

if __name__ == "__main__":
    main()
