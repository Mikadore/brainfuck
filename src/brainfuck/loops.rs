use crate::ast::Instruction;

use super::ast;
use rayon::prelude::*;

//Cannot be parallelized by rayon as we rely on sequential `push`s to `out`
fn unroll_into<'a>(ast: &mut impl Iterator<Item=&'a ast::Code>, out: &mut Vec<ast::Instruction>)  {
    ast
    .for_each(|c| {
        match c {
            ast::Code::Loop(lp) => {
                out.push(ast::Instruction::Lps);
                unroll_into(&mut lp.iter(), out);
                out.push(ast::Instruction::Lpe);
            },
            ast::Code::Instructions(inst)  => {
                for i in inst {
                    out.push(*i);
                }
            }
        }
    });
}

fn link(src: &mut Vec<ast::Instruction>) {
    let mut state = Vec::new();
    src
        .par_iter()
        .enumerate()
        .filter_map(|a|{
            if let (_, ast::Instruction::Lpe | ast::Instruction::Lps) = a {
                Some((a.0, *a.1))
            } else {
                None
            }
        })
        .collect::<Vec::<_>>()  //need to collect as converting a rayon iterator to a normal one isn't solved yet
        .into_iter()
        .for_each(|item|{
            match item {
                (i, ast::Instruction::Lps) => {
                    state.push(i);
                },
                (i, ast::Instruction::Lpe) => {
                    let j = state.pop().unwrap();
                    src[j] = Instruction::Jmpc(i);
                    src[i] = Instruction::Jmpc(j+1);
                },
                _ => {}
            }
        });
}

pub fn unroll(ast: &[ast::Code]) -> Vec<ast::Instruction> {
    let mut out = Vec::new();
    unroll_into(&mut ast.iter(), &mut out);
    link(&mut out);
    out
}