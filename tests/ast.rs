#[test]
fn ast_helloworld() -> std::io::Result<()> {
    let program = include_str!("data/helloworld.bf");
    let output = include_str!("data/helloast");

    let tokens = bf::ast::tokenize(program);
    let ast = bf::ast::build_ast(&mut bf::ast::combine(&mut tokens.into_iter()).into_iter());

    let mut out = std::io::Cursor::new(Vec::new());

    bf::ast::dump(&ast, &mut out)?;

    assert_eq!(output.as_bytes(), out.get_ref());

    Ok(())
}