#![feature(or_patterns)]

use std::io::{Read, Write};
use itertools::Itertools;
use clap::{App, Arg};

enum Instruction {
    Lpe,
    Lps,
    Incr,
    Decr,
    Datg,
    Datp,
    Mvr,
    Mvl,
    Chgc(isize),
    Mvip(isize),
}
impl Instruction {
    fn val(&self) -> isize {
        match self {
            Instruction::Incr =>  1,
            Instruction::Decr => -1,
            Instruction::Mvr  =>  1,
            Instruction::Mvl  => -1,

            Instruction::Chgc(n) => *n,
            Instruction::Mvip(n) => *n, 

            _ => 0
        }
    }
}
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;
        let s;
        write!(f, "{}", match *self {
            Lpe         => "Lpe",
            Lps         => "Lps",
            Incr        => "Incr", 
            Decr        => "Decr",
            Datg        => "Datg",
            Datp        => "Datp",
            Mvr         => "Mvr", 
            Mvl         => "Mvl",
            Chgc(n)     => { s = format!("Chgc {}", n); &s }
            Mvip(n)     => { s = format!("Mvip {}", n); &s }
        })   
    }
}

fn main() -> std::io::Result<()> {
    let args = App::new("BFII")
                    .author("Mikdore <admin@mikadore.eu>")
                    .arg(   
                        Arg::with_name("FILE")
                            .help("The brainfuck file to use")
                            .index(1)
                    )
                    .arg(
                        Arg::with_name("TOKEN_FILE")
                            .help("(Optional) specifies a file into which tokenized bf is dumped")
                            .short("t")
                            .long("tokfile")
                            .takes_value(true)
                    )
                    .get_matches();

    let code: Vec<Instruction> = if let Some(file) = args.value_of("FILE") {
       std::fs::read_to_string(file)?
    } else {
        let mut str = String::new();
        std::io::stdin()
            .lock()
            .read_to_string(&mut str)?;
        str
    }
    .chars()
    .into_iter()
    .filter_map(|c|{
        match c {
            '+' => Some(Instruction::Incr),
            '-' => Some(Instruction::Decr),
            '<' => Some(Instruction::Mvl),
            '>' => Some(Instruction::Mvr),
            ',' => Some(Instruction::Datg),
            '.' => Some(Instruction::Datp),
            '[' => Some(Instruction::Lps),
            ']' => Some(Instruction::Lpe),
            _ => None
        }
    })
    .coalesce(|a,b| {
        use Instruction::*;
        match (&a, &b) {
            (Incr | Decr | Chgc(_), Incr | Decr | Chgc(_)) => {
                Ok(Chgc(a.val() + b.val()))
            },
            (Mvr | Mvl | Mvip(_), Mvr | Mvl | Mvip(_)) => {
                Ok(Mvip(a.val() + b.val()))
            }
            _ => {
                Err((a,b))
            }
        }
    })
    .collect();

    if let Some(filename) = args.value_of("TOKEN_FILE")  {
        println!("Dumping tokenized input to {}", filename);
        let mut writer = std::io::BufWriter::new(std::fs::File::create(filename)?);
        
        for instruction in code {
            writer.write_all(format!("{}\n", instruction).as_bytes())?;
        }
    }

    Ok(())
 }
