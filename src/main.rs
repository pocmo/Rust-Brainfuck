use std::collections::HashMap;
use std::vec::Vec;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::error::Error;

fn build_loop_map(tokens: &Vec<char>) -> (HashMap<usize,usize>, HashMap<usize,usize>) {
    let mut forward_map = HashMap::new();
    let mut backward_map = HashMap::new();

    let mut map_stack = Vec::new();

    for i in 0..tokens.len() {
        match tokens[i] {
            '[' => map_stack.push(i),
            ']' => {
                let start = match map_stack.pop() {
                    None => break,
                    Some(x) => x,
                };
                forward_map.insert(start, i);
                backward_map.insert(i, start);
            }
            _ => {}
        }
    }

    return (forward_map, backward_map)
}

fn read_file(filename: &String) -> String {
    let mut file = match File::open(filename) {
        Err(why) => panic!("Could not open file {}: {}", filename, Error::description(&why)),
        Ok(file) => file
    };

    let mut program_code = String::new();
    match file.read_to_string(&mut program_code) {
        Err(why) => panic!("Could not read file {}: {}", filename, Error::description(&why)),
        Ok(_) => {}
    };

    return program_code;
}

fn read_filename_from_arguments() -> String {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: cargo run yourfile.bf");
    }

    return args[1].clone();
}

fn evaluate(program_code: String) {
    let mut cells : [i8; 30000] = [0; 30000];
    let tokens = program_code.chars().collect();

    let (forward_map, backward_map) = build_loop_map(&tokens);
    let mut code_pointer = 0;
    let mut cell_pointer = 0;

    while code_pointer < tokens.len() {
        match tokens[code_pointer] {
            '+' => cells[cell_pointer] += 1,
            '-' => cells[cell_pointer] -= 1,
            '>' => cell_pointer += 1,
            '<' => cell_pointer -= 1,
            '[' => {
                if cells[cell_pointer] == 0 {
                    match forward_map.get(&code_pointer) {
                        Some(position) => code_pointer = *position,
                        _ => {}
                    }
                }
            },
            ']' => {
                if cells[cell_pointer] != 0 {
                    match backward_map.get(&code_pointer) {
                        Some(position) => code_pointer = *position,
                        _ => {}
                    }
                }
            },
            '.' => print!("{}", (cells[cell_pointer] as u8) as char),
            _ => {}
        }
        code_pointer += 1;
    }
}

fn main() {
    let filename = read_filename_from_arguments();
    let program_code = read_file(&filename);
    evaluate(program_code);
}
