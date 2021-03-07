#[test]
fn loop_optimize() {
    /*
    +++++
    [-]
    [+]

    should collapse to 

    Setc(0)

    */    

    use bf::ast::*;
    let mut ast = vec![
        Code::Instructions(vec![Instruction::Chgc(5)]),
        Code::Loop(vec![
            Code::Instructions(vec![Instruction::Chgc(-1)])
        ]),
        Code::Loop(vec![
            Code::Instructions(vec![Instruction::Chgc(1)])
        ])
    ];

    ast = bf::loops::loop_optimize(ast);

    assert!(ast.len() == 1);
    
    if let Code::Instructions(i) = &ast[0] {
        assert!(i.len() == 1);
        if let Instruction::Setc(n) = i[0] {
            assert!(n == 0);
        } else { panic!("Not Setc") }
    } else { panic!("Not Instructions") }
}
