use std::fs;
use std::fmt::Write;
use std::io::Write as StdioWrite;
use std::process::{Command, Stdio};

fn main() {
    splice_inputs(read_input_file("./input"))
        .iter()
        .for_each(self::run_cargo_with_input);
}

fn run_cargo_with_input(input: &String) {
    let mut ps = Command::new("cargo")
    .arg("run")
    .stdin(Stdio::piped())
    .spawn()
    .expect("Cannot execute: cargo run");
    ps.stdin.as_mut().unwrap().write_all(input.as_bytes()).expect("Failed to write to stdin");
    let output = ps.wait_with_output().expect("Failed to read stdout");
    println!("{:?}", output.stdout);
}

fn splice_inputs(entire_input: String) -> Vec<String> {
    let mut input_buf = String::new();
    let mut cases = vec![];
    for line in entire_input.lines().map(str::trim) {
        if line.is_empty() {
            input_buf.pop();
            cases.push(input_buf.clone());
            input_buf = String::new();
        } else {
            writeln!(input_buf, "{}", line).unwrap();
        }
    }
    if !input_buf.is_empty() {
        input_buf.pop();
        cases.push(input_buf);
    }
    cases
}

fn read_input_file(path: &str) -> String {
    fs::read_to_string(path).expect(format!("Failed to read {}", path).as_str())
}

#[test]
fn read_input_file_test() {
    let buf = read_input_file("./input");
    println!("{}", buf);
    assert!(buf.len() > 0);
}

#[test]
fn splice_inputs_test() {
    let buf = read_input_file("./input");
    let v = splice_inputs(buf);
    println!("{:?}", v);
}
