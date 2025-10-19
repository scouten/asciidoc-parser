#!/usr/bin/env python3
import os
import re
import sys

# Locate all crate roots dynamically.

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(__file__)))

def find_source_roots(root):
    """Find all subdirectories that contain Cargo.toml + src/."""
    src_dirs = []
    for dirpath, dirnames, filenames in os.walk(root):
        if "Cargo.toml" in filenames and "src" in dirnames:
            src_dirs.append(os.path.join(dirpath, "src"))
    return src_dirs

SRC_DIRS = find_source_roots(ROOT)
if not SRC_DIRS:
    print("⚠️  No src directories found under repository root", file=sys.stderr)
else:
    print(f"🔎 Found source roots: {', '.join(SRC_DIRS)}", file=sys.stderr)

TEST_FN_PATTERN = re.compile(r"^\s*#\[test\]\s*$")
FN_DEF_PATTERN = re.compile(r"^\s*fn\s+([A-Za-z0-9_]+)")

# Matches #[cfg(test)] with optional spaces.
TEST_MOD_PATTERN = re.compile(r"^\s*#\[cfg\s*\(\s*test\s*\)\s*\]")

# Matches mod tests { possibly with spaces.
MOD_DEF_PATTERN = re.compile(r"^\s*mod\s+tests\s*\{")

def normalize_path(p: str) -> str:
    """Normalize LCOV and filesystem paths for consistent matching."""
    if "/src/" in p:
        return p[p.find("/src/") :]
    return os.path.basename(p)

def find_test_functions():
    """Return a set of #[test] function names across all src directories."""
    tests = set()
    for src_dir in SRC_DIRS:
        for dirpath, _, filenames in os.walk(src_dir):
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

def find_test_module_ranges():
    """Return {filepath: [(start_line, end_line), ...]} for #[cfg(test)] mod tests blocks."""
    mod_ranges = {}
    for src_dir in SRC_DIRS:
        for dirpath, _, filenames in os.walk(src_dir):
            for fname in filenames:
                if not fname.endswith(".rs"):
                    continue
                path = os.path.join(dirpath, fname)
                with open(path, "r", encoding="utf-8") as f:
                    lines = f.readlines()

                i = 0
                ranges = []
                while i < len(lines):

                    # Look for #[cfg(test)].
                    if TEST_MOD_PATTERN.match(lines[i]):

                        # Look ahead up to 5 lines for "mod tests {".
                        for j in range(i + 1, min(i + 6, len(lines))):
                            if MOD_DEF_PATTERN.match(lines[j]):
                                start = j + 1
                                depth = 1
                                k = start
                                while k < len(lines) and depth > 0:
                                    depth += lines[k].count("{")
                                    depth -= lines[k].count("}")
                                    k += 1
                                end = k
                                ranges.append((start + 1, end))
                                i = k
                                break
                    i += 1
                if ranges:
                    mod_ranges[normalize_path(path)] = ranges
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
            current_file = normalize_path(line.strip().split("SF:")[1])
            skip_block = "/tests/" in current_file or current_file.endswith("/tests.rs")
            if skip_block:
                excluded_files.add(current_file)
        elif line.startswith("end_of_record"):
            skip_block = False

        if skip_block:
            continue

        # Remove function-level data.
        if line.startswith(("FN:", "FNDA:")):
            fn_name = line.strip().split(",")[-1]
            if fn_name in test_funcs:
                excluded_fns += 1
                continue
            m = re.match(r"FN:(\d+),", line)
            if m and in_test_module(current_file, int(m.group(1)), test_mod_ranges):
                excluded_fns += 1
                continue

        # Remove line coverage inside test modules.
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

    with open("lcov.info", "r", encoding="utf-8") as infile, open(
        "lcov.filtered.info", "w", encoding="utf-8"
    ) as outfile:
        excluded_files, excluded_fns, excluded_lines = filter_lcov(
            infile, outfile, test_funcs, test_mod_ranges
        )

    print(f"🧹 Excluded {len(excluded_files)} test files", file=sys.stderr)
    print(f"🧹 Excluded {excluded_fns} test functions", file=sys.stderr)
    print(f"🧹 Excluded {excluded_lines} test lines", file=sys.stderr)
    print("✅ Wrote filtered LCOV to lcov.filtered.info", file=sys.stderr)

if __name__ == "__main__":
    main()
