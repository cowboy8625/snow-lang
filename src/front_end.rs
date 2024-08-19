#![allow(dead_code)]
#![allow(warnings)]
use logos::Logos;

// ----------------------------------
// Lexer
// ----------------------------------

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[\f]+")]
pub enum Token {
    Error,
    #[token("enum")]
    Enum,
    #[token("match")]
    Match,
    #[token("if")]
    If,
    #[token("then")]
    Then,
    #[token("else")]
    Else,
    #[token("on")]
    On,
    #[token("->")]
    Arrow,
    #[token("and")]
    And,
    #[token("(")]
    ParenL,
    #[token(")")]
    ParenR,
    #[token("{")]
    BraceL,
    #[token("}")]
    BraceR,
    #[token("[")]
    BracketL,
    #[token("]")]
    BracketR,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("|")]
    Pipe,
    #[token("=")]
    Equals,
    #[token(">")]
    GreaterThan,
    #[token("<")]
    LessThan,
    #[token(">=")]
    GreaterThanOrEqual,
    #[token("<=")]
    LessThanOrEqual,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("!")]
    Bang,
    #[regex(r#"'[^']'"#)]
    CharLiteral,
    #[regex(r#"[0-9]+"#, |lex| lex.slice().to_string())]
    IntLiteral(String),
    #[regex(r#"[a-zA-Z_][a-zA-Z0-9_]*"#, |lex| lex.slice().to_string())]
    Identifier(String),
    #[token("\n")]
    NewLine,
    #[regex(r#"[ \t\r]*"#)]
    Whitespace,
    #[regex(r#"--.*\n"#, |lex| lex.slice().to_string())]
    Comment(String),
    FunctionName(String),
}

// -----------------------
// AST
// -----------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atom(Atom),
    Identifier(String),
    PrefixOp {
        op: Operator,
        right: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    Function {
        name: String,
        params: Vec<String>,
        signature: Vec<String>,
        body: Box<Expr>,
    },
    Enum {
        name: String,
        type_args: Vec<String>,
        variants: Vec<Vec<Expr>>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Int(i32),
    Char(char),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Not,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

// -----------------------
// Parser
// -----------------------

pub struct Parser<I>
where
    I: Iterator<Item = Result<Token, ()>>,
{
    tokens: std::iter::Peekable<I>,
    current: Option<Token>,
    last_token: Option<Token>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Result<Token, ()>>,
{
    pub fn new(mut tokens: std::iter::Peekable<I>) -> Self {
        let current = None;
        let mut parser = Self {
            tokens,
            current,
            last_token: None,
        };
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        use Token::*;
        loop {
            let Some(token) = self.tokens.next().and_then(|res| res.ok()) else {
                self.current = None;
                break;
            };
            match (self.last_token.clone(), token.clone()) {
                (None | Some(NewLine), Identifier(name)) => {
                    self.last_token = Some(token);
                    self.current = Some(FunctionName(name));
                    break;
                }
                (_, Whitespace | NewLine | Comment(_)) => {}
                _ => {
                    self.current = Some(token.clone());
                    self.last_token = Some(token);
                    break;
                }
            }

            self.last_token = Some(token);
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        match self.current.clone() {
            Some(actual) if actual == expected => {
                self.next_token();
                Ok(())
            }
            Some(actual) => {
                Err(format!("Expected {:?}, but found {:?}", expected, actual))
            }
            _ => Err("Unexpected end of input".into()),
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        match self.tokens.peek() {
            Some(Ok(token)) => Some(token),
            Some(Err(_)) => panic!("Unexpected error"),
            _ => None,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, Vec<String>> {
        let mut exprs = Vec::new();
        let mut errors = Vec::new();
        loop {
            if self.peek() == None || !errors.is_empty() {
                // TODO: soak up remaining tokens and start parsing where we can pick back up
                break;
            }
            match self.parse_expression() {
                Ok(expr) => exprs.push(expr),
                Err(error) => errors.push(error),
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(exprs)
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        match self.current.clone() {
            Some(Token::If) => self.parse_if(),
            Some(Token::Enum) => self.parse_enum(),
            Some(Token::FunctionName(_)) => self.parse_function(),
            _ => self.parse_binary_op(0),
        }
    }

    fn parse_if(&mut self) -> Result<Expr, String> {
        self.expect(Token::If)?;
        let condition = self.parse_expression()?;
        self.expect(Token::Then)?;
        let then_branch = self.parse_expression()?;
        let else_branch = if let Some(Token::Else) = self.current {
            self.next_token();
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        Ok(Expr::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn parse_enum(&mut self) -> Result<Expr, String> {
        self.expect(Token::Enum)?;

        let enum_name = match self.current.clone() {
            Some(Token::Identifier(name)) => {
                self.next_token();
                name
            }
            _ => return Err("Expected identifier after 'enum'".into()),
        };

        let mut type_args = Vec::new();
        while let Some(Token::Identifier(type_arg)) = self.current.clone() {
            type_args.push(type_arg);
            self.next_token();
        }

        self.expect(Token::Equals)?;

        let mut variants = Vec::new();
        while let Some(Token::Identifier(variant_name)) = self.current.clone() {
            self.next_token();

            let mut variant = vec![Expr::Identifier(variant_name)];
            while let Some(Token::Identifier(type_name)) = self.current.clone() {
                eprintln!("Parsing enum variant: {:?}", type_name);
                variant.push(Expr::Identifier(type_name));
                self.next_token();
            }

            variants.push(variant);

            if self.expect(Token::Pipe).is_err() {
                break;
            }
        }

        Ok(Expr::Enum {
            name: enum_name,
            type_args,
            variants,
        })
    }

    fn parse_function(&mut self) -> Result<Expr, String> {
        let Token::FunctionName(name) = self.current.clone().unwrap() else {
            unreachable!();
        };
        self.next_token();
        let mut params = Vec::new();
        while let Some(Token::Identifier(param)) = self.current.clone() {
            params.push(param);
            self.next_token();
        }
        self.expect(Token::Colon)?;
        let mut signature = Vec::new();
        while let Some(Token::Identifier(return_type)) = self.current.clone() {
            signature.push(return_type);
            self.next_token();
            if self.expect(Token::Arrow).is_err() {
                break;
            }
        }
        self.expect(Token::Equals)?;
        let body = self.parse_expression()?;
        Ok(Expr::Function {
            name,
            params,
            signature,
            body: Box::new(body),
        })
    }

    fn parse_binary_op(&mut self, min_prec: u8) -> Result<Expr, String> {
        let mut left = self.parse_prefix_op()?;

        while let Some(op) = self.current_operator() {
            let prec = self.get_precedence(&op);
            if prec < min_prec {
                break;
            }

            self.next_token();
            let right = self.parse_binary_op(prec + 1)?;

            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_prefix_op(&mut self) -> Result<Expr, String> {
        if let Some(op) = self.current_prefix_operator() {
            self.next_token(); // Consume operator
            let right = self.parse_prefix_op()?;
            return Ok(Expr::PrefixOp {
                op,
                right: Box::new(right),
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current.clone() {
            Some(Token::Identifier(name)) => {
                self.next_token();
                Ok(Expr::Identifier(name))
            }
            Some(Token::IntLiteral(number)) => {
                self.next_token();
                Ok(Expr::Atom(Atom::Int(number.parse::<i32>().unwrap())))
            }
            _ => Err(format!(
                "Unexpected token in primary expression {:?}",
                self.current
            )),
        }
    }

    fn current_operator(&self) -> Option<Operator> {
        match self.current {
            Some(Token::Plus) => Some(Operator::Add),
            Some(Token::Minus) => Some(Operator::Sub),
            Some(Token::Asterisk) => Some(Operator::Mul),
            Some(Token::Slash) => Some(Operator::Div),
            Some(Token::Percent) => Some(Operator::Mod),
            Some(Token::GreaterThan) => Some(Operator::GreaterThan),
            Some(Token::LessThan) => Some(Operator::LessThan),
            Some(Token::GreaterThanOrEqual) => Some(Operator::GreaterThanOrEqual),
            Some(Token::LessThanOrEqual) => Some(Operator::LessThanOrEqual),
            Some(Token::Equal) => Some(Operator::Equal),
            _ => None,
        }
    }

    fn current_prefix_operator(&self) -> Option<Operator> {
        match self.current {
            Some(Token::Minus) => Some(Operator::Sub),
            Some(Token::Bang) => Some(Operator::Not),
            _ => None,
        }
    }

    fn get_precedence(&self, op: &Operator) -> u8 {
        match op {
            Operator::Or => 1,
            Operator::And => 2,
            Operator::Equal | Operator::NotEqual => 3,
            Operator::LessThan
            | Operator::GreaterThan
            | Operator::LessThanOrEqual
            | Operator::GreaterThanOrEqual => 4,
            Operator::Add | Operator::Sub => 5,
            Operator::Mul | Operator::Div | Operator::Mod => 6,
            Operator::Not => 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::front_end::Token;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_lexer_with_spacing() {
        let input = r#"Int->Int->Int"#;
        let lexer = Token::lexer(input);
        let tokens = lexer.collect::<Vec<_>>();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::Identifier("Int".to_string(),),),
                Ok(Token::Arrow),
                Ok(Token::Identifier("Int".to_string(),),),
                Ok(Token::Arrow),
                Ok(Token::Identifier("Int".to_string(),),)
            ]
        );
        let input = r#"Int -> Int -> Int"#;
        let lexer = Token::lexer(input);
        let tokens = lexer.collect::<Vec<_>>();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::Identifier("Int".to_string(),),),
                Ok(Token::Whitespace),
                Ok(Token::Arrow),
                Ok(Token::Whitespace),
                Ok(Token::Identifier("Int".to_string(),),),
                Ok(Token::Whitespace),
                Ok(Token::Arrow),
                Ok(Token::Whitespace),
                Ok(Token::Identifier("Int".to_string(),),)
            ]
        );
    }

    #[test]
    fn test_lexer() {
        let input = r#"
max x y
    : Int -> Int -> Int
    = if x > y then x else y
        "#;
        let lexer = Token::lexer(input);
        assert_eq!(
            lexer.collect::<Vec<_>>(),
            vec![
                Ok(Token::NewLine),
                Ok(Token::Identifier("max".to_string())),
                Ok(Token::Whitespace),
                Ok(Token::Identifier("x".to_string())),
                Ok(Token::Whitespace),
                Ok(Token::Identifier("y".to_string())),
                Ok(Token::NewLine),
                Ok(Token::Whitespace),
                Ok(Token::Colon),
                Ok(Token::Whitespace),
                Ok(Token::Identifier("Int".to_string())),
                Ok(Token::Whitespace),
                Ok(Token::Arrow),
                Ok(Token::Whitespace),
                Ok(Token::Identifier("Int".to_string())),
                Ok(Token::Whitespace),
                Ok(Token::Arrow),
                Ok(Token::Whitespace),
                Ok(Token::Identifier("Int".to_string())),
                Ok(Token::NewLine),
                Ok(Token::Whitespace),
                Ok(Token::Equals),
                Ok(Token::Whitespace),
                Ok(Token::If),
                Ok(Token::Whitespace),
                Ok(Token::Identifier("x".to_string())),
                Ok(Token::Whitespace),
                Ok(Token::GreaterThan),
                Ok(Token::Whitespace),
                Ok(Token::Identifier("y".to_string())),
                Ok(Token::Whitespace),
                Ok(Token::Then),
                Ok(Token::Whitespace),
                Ok(Token::Identifier("x".to_string())),
                Ok(Token::Whitespace),
                Ok(Token::Else),
                Ok(Token::Whitespace),
                Ok(Token::Identifier("y".to_string())),
                Ok(Token::NewLine),
                Ok(Token::Whitespace),
            ]
        );
    }

    #[test]
    fn test_parser_function() {
        let input = r#"
max x y
    : Int -> Int -> Int
    = if x > y then x else y
        "#;
        let lexer = Token::lexer(input);
        let mut parser = Parser::new(lexer.peekable());
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(errors) => {
                for e in errors {
                    eprintln!("Error: {}", e);
                }
                panic!();
            }
        };

        assert_eq!(ast.len(), 1);
        assert_eq!(
            ast[0],
            Expr::Function {
                name: "max".to_string(),
                params: vec!["x".to_string(), "y".to_string()],
                signature: vec!["Int".to_string(), "Int".to_string(), "Int".to_string()],
                body: Box::new(Expr::If {
                    condition: Box::new(Expr::BinaryOp {
                        left: Box::new(Expr::Identifier("x".to_string())),
                        op: Operator::GreaterThan,
                        right: Box::new(Expr::Identifier("y".to_string()))
                    }),
                    then_branch: Box::new(Expr::Identifier("x".to_string())),
                    else_branch: Some(Box::new(Expr::Identifier("y".to_string())))
                })
            }
        );
    }

    #[test]
    fn test_parser_enum() {
        let input = r#"
            enum Option a
                = Some a
                | None
        "#;
        let lexer = Token::lexer(input);
        let mut parser = Parser::new(lexer.peekable());
        let ast = parser.parse();

        assert!(ast.is_ok());
        assert_eq!(ast.clone().unwrap().len(), 1);
        assert_eq!(
            ast.unwrap()[0],
            Expr::Enum {
                name: "Option".to_string(),
                type_args: vec!["a".to_string()],
                variants: vec![
                    vec![
                        Expr::Identifier("Some".to_string()),
                        Expr::Identifier("a".to_string())
                    ],
                    vec![Expr::Identifier("None".to_string())]
                ]
            }
        );
    }
}
