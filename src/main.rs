use std::collections::HashMap;
use std::vec::Vec;

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

fn main() {
    let program_code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string();

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
