use super::ast;
use super::loops;
use std::io::{Read, Write};
use std::num::Wrapping;

pub struct Interpreter {}


impl Interpreter {
    pub fn run(ast: &[ast::Code], stdin: &mut impl Read, stdout: &mut impl Write) -> std::io::Result<()>{
        let instructions = loops::unroll(&ast);   
        let mut data = vec![Wrapping(0u8);1024*1024]; 

        let mut stdin = stdin.bytes();
        
        let mut data_ptr = 0usize;
        let mut instruction_ptr = 0usize;


        use ast::Instruction::*;

        while instruction_ptr < instructions.len() {
            match instructions[instruction_ptr] {
                Jmpc(i) => {
                    if data[data_ptr] != Wrapping(0) {
                        instruction_ptr = i;
                        continue;
                    }
                },
                Chgc(n) => {
                    data[data_ptr] = if n > 0 {
                        data[data_ptr] + Wrapping(n as u8)
                    } else {
                        data[data_ptr] - Wrapping(n.abs() as u8)
                    }
                },
                Mvcp(n) => {
                    data_ptr = if n > 0 {
                        data_ptr + (n as usize)
                    } else {
                        data_ptr - (n.abs() as usize)
                    }
                },
                Setc(n) => {
                    data[data_ptr] = Wrapping(n);
                },
                Dtcg => {
                    data[data_ptr] = Wrapping(stdin.next().unwrap_or(Ok(0u8))?);
                },
                Dtcp => {
                    stdout.write_all(&[data[data_ptr].0])?;
                }
                Lpe | Lps => { }
            }

            instruction_ptr += 1;
        }
        Ok(())
    }
}