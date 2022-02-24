use super::lexer::Lexer;
use super::token::{BlockType, KeyWord, Token};
#[test]
fn do_block() {
    let src = "main = do
    print 'c'
    print \"Hello World\"
    print 2134
";
    let right = vec![
        (Token::OpenBlock(BlockType::Fn), 5..5),
        (Token::Id("main".into()), 0..4),
        (Token::Op("=".into()), 5..6),
        (Token::OpenBlock(BlockType::Do), 7..9),
        (Token::KeyWord(KeyWord::Print), 14..19),
        (Token::Char('c'), 20..23),
        (Token::KeyWord(KeyWord::Print), 28..33),
        (Token::String("Hello World".into()), 34..47),
        (Token::KeyWord(KeyWord::Print), 52..57),
        (Token::Int("2134".into()), 58..62),
        (Token::CloseBlock(BlockType::Do), 63..63),
        (Token::CloseBlock(BlockType::Fn), 63..63),
    ];
    let left = Lexer::new(src).parse().tokens;
    assert_eq!(left, right);
}

#[test]
fn function() {
    let src = "add x y = x + y

sub x y =
    x - y

main = print (add 1 2)
";
    let right = vec![
        (Token::OpenBlock(BlockType::Fn), 8..8),
        (
            Token::IdList(vec![
                ("add".into(), 0..3),
                ("x".into(), 4..5),
                ("y".into(), 6..7),
            ]),
            0..7,
        ),
        (Token::Op("=".into()), 8..9),
        (Token::Id("x".into()), 10..11),
        (Token::Op("+".into()), 12..13),
        (Token::Id("y".into()), 14..15),
        (Token::CloseBlock(BlockType::Fn), 16..17),
        (Token::OpenBlock(BlockType::Fn), 25..25),
        (
            Token::IdList(vec![
                ("sub".into(), 17..20),
                ("x".into(), 21..22),
                ("y".into(), 23..24),
            ]),
            17..24,
        ),
        (Token::Op("=".into()), 25..26),
        (Token::Id("x".into()), 31..32),
        (Token::Op("-".into()), 33..34),
        (Token::Id("y".into()), 35..36),
        (Token::CloseBlock(BlockType::Fn), 37..38),
        (Token::OpenBlock(BlockType::Fn), 43..43),
        (Token::Id("main".into()), 38..42),
        (Token::Op("=".into()), 43..44),
        (Token::KeyWord(KeyWord::Print), 45..50),
        (Token::Ctrl('('), 51..52),
        (Token::Id("add".into()), 52..55),
        (Token::Int("1".into()), 56..57),
        (Token::Int("2".into()), 58..59),
        (Token::Ctrl(')'), 59..60),
        (Token::CloseBlock(BlockType::Fn), 61..61),
    ];
    let left = Lexer::new(src).parse().tokens;
    assert_eq!(left, right);
}
