// use snowc_lexer::Scanner;
// use snowc_parse::{App, Atom, Binary, Expr, TypeInfo, Unary};
use snowc_parse::Expr;
use snowc_vm::{Item, Label, Span, Text, Token, TokenOp};

pub fn gen_code(input: &Vec<Expr>) -> Vec<u8> {
    let program = vec![
        // .entry main
        Item::EntryPoint(Token::Id("main".into(), Span::default())),
        Item::Data(vec![]),
        // .text
        Item::Text(vec![
            // main:
            //     load %0 1 ; a
            Text::new_opcode_with_label("main", TokenOp::Load(0, 1, 0)),
            //     load %1 1 ; b
            Text::new_opcode(TokenOp::Load(1, 1, 0)),
            //     load %2 46
            Text::new_opcode(TokenOp::Load(2, 46, 0)),
            // loop:
            //     push %1
            Text::new_opcode_with_label("loop", TokenOp::Push(1)),
            //     add %0 %1 %1
            Text::new_opcode(TokenOp::Add(0, 1, 1)),
            //     pop %0
            Text::new_opcode(TokenOp::Pop(0)),
            //     inc %3
            Text::new_opcode(TokenOp::Inc(3)),
            //     eq %3 %2
            Text::new_opcode(TokenOp::Eq(3, 2)),
            //     jne loop
            Text::new_opcode(TokenOp::Jne(Label {
                name: "loop".into(),
                span: Span::default(),
                def: false,
            })),
            //     prti %0
            Text::new_opcode(TokenOp::Prti(0)),
            //     hlt
            Text::new_opcode(TokenOp::Hlt),
        ]),
    ];
    let program = snowc_vm::assemble_from_ast(&program).unwrap();
    for item in program.chunks(4) {
        println!("{:?}", item);
    }
    program
}
