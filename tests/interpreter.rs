#[test]
fn hello_out() -> std::io::Result<()> {
    let program = include_str!("data/helloworld.bf");
    let output = include_bytes!("data/helloout");

    let mut stdin  = std::io::Cursor::new(Vec::new());
    let mut stdout = std::io::Cursor::new(Vec::new());

    bf::run(program, &mut stdin, &mut stdout)?;

    assert_eq!(stdout.get_ref(), output);

    Ok(())
}
#[test]
fn mandel_out() -> std::io::Result<()> {
    let program = include_str!("data/mandelbrot.bf");
    let output = include_bytes!("data/mandelout");

    let mut stdin  = std::io::Cursor::new(Vec::new());
    let mut stdout = std::io::Cursor::new(Vec::with_capacity(output.len()));

    bf::run(program, &mut stdin, &mut stdout)?;

    assert_eq!(stdout.get_ref(), output);

    Ok(())
}