use std::env;
use std::fs::File;
use std::io::prelude::*;

fn run(ops: Vec<char>, memory: &mut Vec<u8>, pointer: &mut usize) {
    let mut stack: Vec<usize> = Vec::new();
    let length = ops.len();
    let mut i: usize = 0;
    loop {
        match ops[i] {
            '[' => {
                stack.push(i + 1);
                i += 1
            }
            ']' => match memory[*pointer] {
                0 => {
                    stack.pop();
                    i += 1
                }
                _ => i = stack[stack.len() - 1],
            },
            ',' => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("failed to read stdin");
                memory[*pointer] = input[0];
                i += 1
            }
            '.' => {
                print!("{}", memory[*pointer] as char);
                i += 1
            }
            '+' => {
                // TODO: 正解がわからない
                memory[*pointer] = memory[*pointer].wrapping_add(1);
                i += 1
            }
            '-' => {
                // TODO: 正解がわからない
                memory[*pointer] = memory[*pointer].wrapping_sub(1);
                i += 1
            }
            '>' => {
                *pointer += 1;
                i += 1
            }
            '<' => {
                *pointer -= 1;
                i += 1
            }
            _ => i += 1,
        }
        if i == length {
            break;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    let mut src = String::new();
    let mut f = File::open(filepath).expect("file not found");
    f.read_to_string(&mut src)
        .expect("something went wrong reading the file");

    let mut memory: Vec<u8> = vec![0; 4096];
    let mut pointer = 2048;
    let ops: Vec<char> = src.chars().collect();
    run(ops, &mut memory, &mut pointer);
}
