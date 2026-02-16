//! Unit tests for cli::commands::info::info_source
//! — migrated from inline tests

use rustean::cli::commands::info::info_source::find_symbol_end;

#[test]
fn find_end_simple_function() {
    let lines = vec![
        "fn example() {",
        "    println!(\"hello\");",
        "}",
        "fn another() {",
    ];
    assert_eq!(find_symbol_end(&lines, 0), 3);
}

#[test]
fn find_end_nested_braces() {
    let lines = vec![
        "fn nested() {",
        "    if true {",
        "        let x = 1;",
        "    }",
        "}",
        "// next line",
    ];
    assert_eq!(find_symbol_end(&lines, 0), 5);
}

#[test]
fn find_end_no_braces() {
    let lines: Vec<&str> =
        (0..11).map(|_| "fn sig();").collect();
    assert_eq!(find_symbol_end(&lines, 0), 10);
}

#[test]
fn find_end_max_lines() {
    let mut lines: Vec<&str> =
        vec!["fn large() {"];
    let body =
        vec!["    let x = 1;"; 100];
    lines.extend(body);
    lines.push("}");
    assert_eq!(find_symbol_end(&lines, 0), 51);
}

#[test]
fn find_end_offset_start() {
    let lines = vec![
        "// line 0",
        "// line 1",
        "fn target() {",
        "    println!(\"test\");",
        "}",
        "// line 5",
    ];
    assert_eq!(find_symbol_end(&lines, 2), 5);
}
