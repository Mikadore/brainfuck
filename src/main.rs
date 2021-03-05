#![feature(or_patterns)]

use std::io::Read;
use clap::{App, Arg};
use bf::interpreter::Interpreter;

fn main() -> std::io::Result<()> {
    let args = App::new("BFII")
                    .author("Mikdore <admin@mikadore.eu>")
                    .arg(   
                        Arg::with_name("FILE")
                            .help("The brainfuck file to use")
                            .index(1)
                    )
                    .arg(
                        Arg::with_name("AST_FILE")
                            .help("(Optional) specifies a file into which the (optimized) ast is dumped")
                            .short("a")
                            .long("astfile")
                            .takes_value(true)
                    )
                    .get_matches();

    let source = if let Some(file) = args.value_of("FILE") {
       std::fs::read_to_string(file)?
    } else {
        let mut str = String::new();
        std::io::stdin()
            .lock()
            .read_to_string(&mut str)?;
        str
    };

    let source  =   bf::ast::tokenize(&source);
    let source  =   bf::ast::combine(&mut source.into_iter());
    let ast     =   bf::ast::build_ast(&mut source.into_iter());

    if let Some(file) = args.value_of("AST_FILE") {
        let mut writer = std::io::BufWriter::new(std::fs::File::create(file)?);
        bf::ast::dump(&ast, &mut writer)?;
    }

    Interpreter::run(
        &ast,
        &mut std::io::stdin().lock(),
        &mut std::io::stdout().lock()
    )?;


    Ok(())
 }
