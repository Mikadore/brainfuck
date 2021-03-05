
use super::*;
use itertools::Itertools;

#[derive(Copy, Debug)]
pub enum Instruction {
    Dtcp,
    Dtcg,
    Lpe,
    Lps,
    Chgc(isize),
    Mvcp(isize),
    Setc(u8),
}
impl Clone for Instruction {
    fn clone(&self) -> Self {
        return *self
    }
}

#[derive(Debug)]
pub enum Code {
    Instructions(Vec::<Instruction>),
    Loop(Vec::<Code>)
} 

pub fn tokenize(src: &str) -> Vec::<Commands> { 
    src
    .chars()
    .into_iter()
    .filter_map(|c|{
        match c {
            '+' => Some(Commands::Incr),
            '-' => Some(Commands::Decr),
            '<' => Some(Commands::Mvl),
            '>' => Some(Commands::Mvr),
            ',' => Some(Commands::Datg),
            '.' => Some(Commands::Datp),
            '[' => Some(Commands::Lps),
            ']' => Some(Commands::Lpe),
            _ => None
        }
    })
    .collect()
}

pub fn combine(src: &mut impl Iterator::<Item=Commands>) -> Vec::<Instruction> {
    src
        .map(|cmd| {
            use Commands::*;
            match cmd {
                Incr | Decr => Instruction::Chgc(cmd.val()),
                Mvl  | Mvr  => Instruction::Mvcp(cmd.val()),
                Datg        => Instruction::Dtcg,
                Datp        => Instruction::Dtcp,
                Lpe         => Instruction::Lpe,
                Lps         => Instruction::Lps
            }
        })
        .coalesce(|a, b|{
            match (&a, &b) {
                (Instruction::Chgc(x), Instruction::Chgc(y)) => Ok(Instruction::Chgc(x + y)),
                (Instruction::Mvcp(x), Instruction::Mvcp(y)) => Ok(Instruction::Mvcp(x + y)),
                _=> Err((a,b))
            }
        })
        .collect()
}
pub fn build_ast(src: &mut impl Iterator::<Item=Instruction>) -> Vec::<Code> {

    let mut vec = Vec::<Code>::new();
    let mut instr = Vec::new();
    while let Some(inst) = src.next() {
        //dbg!(format!("{:?}", inst));
        match inst {
            Instruction::Lps => {

                let mut new = Vec::new();
                std::mem::swap(&mut new, &mut instr);
                vec.push(Code::Instructions(new));
                vec.push(Code::Loop(build_ast(src)));
            },
            Instruction::Lpe => {
                vec.push(Code::Instructions(instr));
                return vec;
            }
            _ => {
                instr.push(inst);
            }
        }
    }
    if !instr.is_empty() {
        vec.push(Code::Instructions(instr))
    }
    vec
}

pub fn dump_tabbed(ast: &Vec::<Code>, w: &mut impl std::io::Write, tablevel: i32) -> std::io::Result<()> {

    for code in ast {
        match code {
            Code::Instructions(i) => {
                for inst in i {
                    for _ in 0..tablevel {
                        w.write_all(b"    ")?;
                    }
                    w.write_all(format!("{:?}\n", inst).as_bytes())?;
                }
            },
           Code::Loop(vc) => {
                for _ in 0..tablevel {
                    w.write_all(b"    ")?;
                }
                w.write_all(b"[\n")?;
                dump_tabbed(vc, w, tablevel + 1)?;
                for _ in 0..tablevel {
                    w.write_all(b"    ")?;
                }
                w.write_all(b"]\n")?;
           }
        }
    }
    Ok(())
}
pub fn dump(ast: &Vec::<Code>, w: &mut impl std::io::Write) -> std::io::Result<()> {
    return dump_tabbed(ast, w, 0);
}