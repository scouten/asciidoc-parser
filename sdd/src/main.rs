// Quick and dirty tool to generate spec coverage for asciidoc-parser. Not
// intended at this time to generalize to any other use case.

// If asciidoc-parser goes well, I may build a more robust version of this at a
// later time. For now, please excuse the hard-coded settings and other
// shortcuts taken.

use std::{collections::HashMap, fs, io::BufRead, path::Path};

use walkdir::{DirEntry, WalkDir};

fn main() {
    let mut spec_coverage: HashMap<String, Vec<(String, bool)>> = HashMap::new();

    let rs_files: Vec<DirEntry> = WalkDir::new("../parser/src/tests")
        .into_iter()
        .filter_entry(|e| {
            if let Some(file_name) = e.file_name().to_str() {
                !file_name.starts_with(".")
            } else {
                false
            }
        })
        .filter_map(|e| {
            let e = e.expect("Directory read error");

            if !e.file_type().is_file() {
                return None;
            }

            if let Some(file_name) = e.file_name().to_str() {
                if file_name.ends_with(".rs") {
                    Some(e)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    for entry in rs_files {
        let path = entry.path();
        if let Some((spec_path, cov)) = parse_rs_file(path) {
            spec_coverage.insert(spec_path, cov);
        }
    }

    println!("{{\n    \"coverage\": {{");

    let adoc_files: Vec<DirEntry> = WalkDir::new("../docs/modules")
        .into_iter()
        .filter_entry(|e| {
            if let Some(file_name) = e.file_name().to_str() {
                !file_name.starts_with(".")
            } else {
                false
            }
        })
        .filter_map(|e| {
            let e = e.expect("Directory read error");

            if !e.file_type().is_file() {
                return None;
            }

            if let Some(file_name) = e.file_name().to_str() {
                if file_name.ends_with(".adoc") {
                    Some(e)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let last_index = adoc_files.len() - 1;

    for (count, entry) in adoc_files.into_iter().enumerate() {
        let path = entry.path().to_str().unwrap().trim_start_matches("../");
        // (unwrap: Should have been filtered out above.)

        println!("        {path:?}: {{");

        emit_adoc_coverage(path, spec_coverage.get(path));

        // println!("            \"1\": 0");
        if count < last_index {
            println!("        }},");
        } else {
            println!("        }}");
        }
    }

    println!("    }}\n}}");
}

fn parse_rs_file(path: &Path) -> Option<(String, Vec<(String, bool)>)> {
    let rs_file = fs::read(path).unwrap();

    let mut tracked_file: Option<String> = None;
    let mut lines: Vec<(String, bool)> = vec![];
    let mut in_non_normative_block = false;
    let mut in_verifies_block = false;

    for line in rs_file.lines() {
        let line = line.unwrap();

        if let Some(tf) = line.strip_prefix("track_file!(\"") {
            if let Some(tf) = tf.strip_suffix("\");") {
                if tracked_file.is_some() {
                    panic!("ERROR: {path:?} contains multiple track_file! macros");
                }
                tracked_file = Some(tf.to_string());
                continue;
            }
        }

        if line.contains("non_normative!(") {
            in_non_normative_block = true;
            in_verifies_block = false;
            continue;
        }

        if line.contains("verifies!(") {
            in_non_normative_block = false;
            in_verifies_block = true;
            continue;
        }

        if line.starts_with("\"#") {
            in_non_normative_block = false;
            in_verifies_block = false;
            continue;
        }

        if line.ends_with("r#\"") {
            continue;
        }

        if in_non_normative_block {
            lines.push((line, false));
        } else if in_verifies_block {
            lines.push((line, true));
        }
    }

    tracked_file.map(|tracked_file| (tracked_file, lines))
}

fn emit_adoc_coverage(path: &str, coverage: Option<&Vec<(String, bool)>>) {
    let path = format!("../{path}");
    let adoc_file = fs::read(path).unwrap();

    let empty_coverage: Vec<(String, bool)> = vec![];
    let coverage = if let Some(coverage) = coverage.as_ref() {
        coverage
    } else {
        &empty_coverage
    };

    let mut coverage_lines = coverage.iter();

    let mut output_lines: Vec<String> = vec![];

    for (count, line) in adoc_file.lines().enumerate() {
        let line = line.unwrap();
        let count = count + 1;

        let coverage_line = coverage_lines.next();

        if line.is_empty() {
            continue;
        }

        if let Some((cov_line, is_normative)) = coverage_line {
            if cov_line == &line && *is_normative {
                output_lines.push(format!("            \"{count}\": 1"));
            }
        } else {
            output_lines.push(format!("            \"{count}\": 0"));
        }
    }

    if output_lines.is_empty() {
        return;
    }

    let last_output_line_index = output_lines.len() - 1;

    for (count, line) in output_lines.iter().enumerate() {
        if count < last_output_line_index {
            println!("{line},");
        } else {
            println!("{line}");
        }
    }
}
