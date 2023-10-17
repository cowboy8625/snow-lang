use super::expr::{Atom, Expr};
use super::op::Op as Oper;
use super::op::Op::*;
use snowc_lexer::{Op, Scanner, Span, Token};

pub fn parse(src: &str) -> Result<Vec<Expr>, Vec<crate::error::Error>> {
    let mut tokens: Vec<Token> = Scanner::new(src).collect();
    let mut ast = Vec::new();
    while tokens.len() > 0 {
        let expr = expression(&mut tokens);
        ast.push(expr);
    }
    Ok(ast)
}

fn expression(tokens: &mut Vec<Token>) -> Expr {
    equality(tokens)
}

fn equality(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = comparison(tokens);
    while let Some(op @ (Eq | Neq)) = get_op(tokens.get(0)) {
        tokens.remove(0);
        let rhs = comparison(tokens);
        let span = Span::from((lhs.span(), rhs.span()));
        lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs), span);
    }
    lhs
}

fn comparison(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = term(tokens);
    while let Some(op @ (Grt | Les | GrtEq | LesEq)) = get_op(tokens.get(0)) {
        tokens.remove(0);
        let rhs = term(tokens);
        let span = Span::from((lhs.span(), rhs.span()));
        lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs), span);
    }
    lhs
}

fn term(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = factor(tokens);
    while let Some(op @ (Plus | Minus)) = get_op(tokens.get(0)) {
        tokens.remove(0);
        let rhs = factor(tokens);
        let span = Span::from((lhs.span(), rhs.span()));
        lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs), span);
    }
    lhs
}

fn factor(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = unary(tokens);
    while let Some(op @ (Mult | Div | Mod)) = get_op(tokens.get(0)) {
        tokens.remove(0);
        let rhs = unary(tokens);
        let span = Span::from((lhs.span(), rhs.span()));
        lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs), span);
    }
    lhs
}

fn unary(tokens: &mut Vec<Token>) -> Expr {
    if let Some(op @ (Minus | Not)) = get_op(tokens.get(0)) {
        let token = tokens.remove(0);
        let rhs = unary(tokens);
        let span = Span::from((token.span(), rhs.span()));
        return Expr::Unary(op, Box::new(rhs), span);
    }
    primary(tokens)
}

fn primary(tokens: &mut Vec<Token>) -> Expr {
    match tokens.remove(0) {
        Token::Int(num) => {
            Expr::Atom(Atom::Int(num.lexme.parse().unwrap_or_default()), num.span)
        }
        Token::Float(float) => Expr::Atom(
            Atom::Float(float.lexme.parse().unwrap_or_default()),
            float.span,
        ),
        Token::Ident(id) => Expr::Atom(Atom::Id(id.lexme), id.span),
        Token::KeyWord(kw) if kw.lexme == "true" => {
            Expr::Atom(Atom::Bool(kw.lexme.parse().unwrap_or_default()), kw.span)
        }
        Token::KeyWord(kw) if kw.lexme == "false" => {
            Expr::Atom(Atom::Bool(kw.lexme.parse().unwrap_or_default()), kw.span)
        }
        Token::Str(string) => Expr::Atom(Atom::String(string.lexme), string.span),
        Token::Char(c) => {
            Expr::Atom(Atom::Char(c.lexme.parse().unwrap_or_default()), c.span)
        }
        _ => unreachable!(),
    }
}

fn get_op(token: Option<&Token>) -> Option<Oper> {
    token.and_then(|t| {
        t.map_op::<Option<Oper>>(|Op { lexme, .. }| crate::op::Op::try_from(lexme).ok())
            .flatten()
    })
}

#[test]
fn parse_test() {
    use pretty_assertions::assert_eq;
    let ast = parse("1 + - 2 * 3");
    for expr in ast.unwrap() {
        assert_eq!(expr.to_string(), "(+ 1 (* (- 2) 3))");
    }
}

// fn parse(tokens: &mut Vec<Token>) -> f64 {
//     let mut value = parse_expression(tokens);
//     while let Some(&Token::Op('+')) | Some(&Token::Op('-')) = tokens.get(0) {
//         match tokens.remove(0) {
//             Token::Op('+') => value += parse_expression(tokens),
//             Token::Op('-') => value -= parse_expression(tokens),
//             _ => {}
//         }
//     }
//     value
// }
//
// fn parse_expression(tokens: &mut Vec<Token>) -> f64 {
//     let mut value = parse_term(tokens);
//     while let Some(&Token::Operator('*')) | Some(&Token::Operator('/')) = tokens.get(0) {
//         match tokens.remove(0) {
//             Token::Operator('*') => value *= parse_term(tokens),
//             Token::Operator('/') => value /= parse_term(tokens),
//             _ => {}
//         }
//     }
//     value
// }
//
// fn parse_term(tokens: &mut Vec<Token>) -> f64 {
//     let operator = if let Some(&Token::Operator(c)) = tokens.get(0) {
//         if c == '-' {
//             tokens.remove(0);
//             Some(c)
//         } else {
//             None
//         }
//     } else {
//         None
//     };
//
//     let mut value = match tokens.remove(0) {
//         Token::Number(num) => num,
//         _ => 0.0,
//     };
//
//     if operator == Some('-') {
//         value = -value;
//     }
//
//     value
//
//     // match tokens.remove(0) {
//     //     Token::Number(num) => num,
//     //     _ => 0.0,
//     // }
// }

// fn main() {
//     loop {
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).expect("Failed to read line");
//         let tokens = lexer(&input);
//         let mut tokens = tokens.iter().copied().collect::<Vec<Token>>();
//         let result = parse(&mut tokens);
//         println!("Result: {}", result);
//     }
// }

// use super::{
//     error::Error,
//     expr::{Atom, Expr},
//     op::Op,
//     precedence::Precedence,
//     Scanner, Span, Token, TokenPosition,
// };
//
// use std::iter::Peekable;
//
// pub struct Parser<'a> {
//     pub(crate) lexer: Peekable<Scanner<'a>>,
//     token_stream: Vec<Token>,
//     errors: Vec<Error>,
// }
//
// impl<'a> Parser<'a> {
//     pub fn new(lexer: Peekable<Scanner<'a>>) -> Self {
//         Self {
//             lexer,
//             token_stream: vec![],
//             errors: Vec::new(),
//         }
//     }
//
//     fn next_if<F: FnOnce(Token) -> bool>(&mut self, func: F) -> Option<Token> {
//         if func(self.peek()) {
//             return Some(self.next());
//         }
//         None
//     }
//
//     fn next(&mut self) -> Token {
//         let token = self
//             .lexer
//             .next()
//             .map(|t| match t {
//                 Token::Error(ch, span) => {
//                     self.report(Error::InvalidChar(
//                         ch.chars().last().unwrap_or('\0'),
//                         span,
//                     ));
//                     Token::Eof(span)
//                 }
//                 _ => t,
//             })
//             .unwrap_or(Token::Eof(Span::default()));
//         self.token_stream.push(token.clone());
//         token
//     }
//
//     fn peek(&mut self) -> Token {
//         self.lexer
//             .peek()
//             .cloned()
//             .unwrap_or(Token::Eof(Span::default()))
//     }
//
//     // fn previous(&self) -> Option<&Token> {
//     //     self.token_stream.last()
//     // }
//
//     fn is_end(&mut self) -> bool {
//         matches!(self.peek(), Token::Eof(..))
//     }
//
//     fn remove_last_error(&mut self) {
//         self.errors.pop();
//     }
//
//     fn report(&mut self, error: Error) -> Expr {
//         let span = error.span();
//         self.errors.push(error);
//         Expr::Error(span)
//     }
//
//     fn recover(&mut self) {
//         loop {
//             let token = self.next();
//             if token.is_op_a(";") || token.is_eof() {
//                 break;
//             }
//         }
//     }
//
//     fn is_delimiter(&mut self, tp: &TokenPosition) -> bool {
//         let next_token_pos = *self.peek().position();
//         matches!(*tp, TokenPosition::End)
//             && (next_token_pos == TokenPosition::Start
//                 || next_token_pos == TokenPosition::End)
//     }
//
//     pub fn parse(mut self) -> Result<Vec<Expr>, Vec<Error>> {
//         let mut ast = vec![];
//         while !self.is_end() {
//             let e = self.declaration();
//             ast.push(e);
//         }
//         if !self.errors.is_empty() || ast.iter().any(|x| x.is_error()) {
//             return Err(self.errors);
//         }
//         Ok(ast)
//     }
//
//     fn is_function(&mut self, token: &Token) -> bool {
//         (token.is_id() && self.peek().is_op_a("="))
//             || (token.is_id() && self.peek().is_id())
//     }
//
//     fn declaration(&mut self) -> Expr {
//         let token = self.next();
//         if self.is_function(&token) {
//             let expr = self.function(&token);
//             if expr.is_error() {
//                 // self.recover(&[Token::Op(";".into(), token.span())]);
//                 self.recover();
//                 return self.declaration();
//             }
//             return expr;
//         } else if token.is_keyword_a("enum") {
//             let expr = self.enum_def(token.span());
//             if expr.is_error() {
//                 // self.recover(&[Token::Op(";".into(), token.span())]);
//                 self.recover();
//                 return self.declaration();
//             }
//             return expr;
//         } else if token.is_id() && self.peek().is_op_a("::") {
//             // } else if let Token::Id(id, span) = token {
//             let expr = self.type_dec(&token);
//             if expr.is_error() {
//                 // self.recover(&[Token::Op(";".into(), token.span())]);
//                 self.recover();
//                 return self.declaration();
//             }
//             return expr;
//         }
//         self.recover();
//         self.report(Error::ItemNotAllowedInGlobalSpace(token.span()))
//     }
//
//     fn type_dec(&mut self, token: &Token) -> Expr {
//         let name = token.value();
//         let start = token.span();
//         if !self.next().is_op_a("::") {
//             return Expr::Error(token.span());
//         }
//         // FIXME:[1](cowboy) types need to currently are only string.
//         //          Moving to a Ident { String, Span } could be
//         //          nice for error messages.
//         // let mut types = vec![];
//         // let mut last_type_span = start;
//         // while let Some(Token::Id(id, tp, span)) = self.next_if(|t| !t.is_op_a(";")) {
//         //     types.push(id.to_string());
//         //     if !self.peek().is_op_a("->") {
//         //         break;
//         //     }
//         //     self.next();
//         //     last_type_span = span;
//         // }
//         //
//         // if !self.peek().is_op_a(";") {
//         //     // FIXME: This probably does not point to correct span
//         //     let span_start = self.peek().span();
//         //     let span = Span::from((span_start, last_type_span));
//         //     return self.report(Error::MissingDeliminator(span));
//         // }
//         // let span_end = self.next().span();
//         // let span = Span::from((start, span_end));
//         // Expr::TypeDec(name.into(), types, span)
//
//         // TODO:
//         // ```hs
//         // add ::
//         //     Int ->
//         //     Int ->
//         //     Int
//         //add x y = x + y
//         // ```
//         let mut types = vec![];
//         let mut last_type_span = start;
//         loop {
//             let Some(Token::Id(id, tp, span)) = self.next_if(|t| !t.is_id()) else {
//                 break;
//             };
//             types.push(id.to_string());
//             last_type_span = span;
//
//             // Check if type is End
//             if self.is_delimiter(&tp) {
//                 break;
//             }
//
//             if !self.peek().is_op_a("->") {
//                 break;
//             }
//             self.next();
//         }
//         let span = Span::from((start, last_type_span));
//         Expr::TypeDec(name.into(), types, span)
//     }
//
//     fn enum_def(&mut self, start: Span) -> Expr {
//         let Token::Id(name, ..) = self.next() else {
//             return Expr::Error(start);
//         };
//
//         let mut variants = vec![];
//         if self.next_if(|t| t.is_op_a("=")).is_some() {
//             while let Token::Id(name, ..) = self.next() {
//                 let mut type_list = vec![];
//                 while let Some(Token::Id(type_id, ..)) = self.next_if(|t| t.is_id()) {
//                     type_list.push(type_id);
//                 }
//                 variants.push((name, type_list));
//                 if self.next_if(|t| t.is_op_a("|")).is_none() {
//                     break;
//                 }
//             }
//         }
//         if !self.peek().is_op_a(";") {
//             // let span = self.peek().span();
//             let end = self.peek().span();
//             let span = Span::from((start, end));
//             return self.report(Error::MissingDeliminator(span));
//         }
//         let end = self.next().span();
//         let span = Span::from((start, end));
//         Expr::Enum(name, variants, span)
//     }
//
//     pub(crate) fn function(&mut self, token: &Token) -> Expr {
//         let start = token.span();
//         let name = token.value();
//         let body = self
//             .expression(Precedence::Fn)
//             .and_then(|lhs| {
//                 let mut args = vec![lhs];
//                 while self.peek().is_id() {
//                     args.push(dbg!(self.expression(Precedence::Fn)));
//                 }
//                 if self.next_if(|t| t.is_op_a("=")).is_none() {
//                     let span = self.peek().span();
//                     return self.report(Error::Unknown(span));
//                 }
//                 let body = self.closure();
//                 args.into_iter().rev().fold(body, |last, next| {
//                     let span = Span::from((last.span(), next.span()));
//                     Expr::Closure(Box::new(next), Box::new(last), span)
//                 })
//             })
//             .or_else(|_| {
//                 self.remove_last_error();
//                 if self.next_if(|t| t.is_op_a("=")).is_none() {
//                     let span = self.peek().span();
//                     return self.report(Error::Unknown(span));
//                 };
//                 self.closure()
//             });
//         let end = self.next().span();
//         let span = Span::from((start, end));
//         Expr::Func(name.into(), Box::new(body), span)
//     }
//
//     pub(crate) fn closure(&mut self) -> Expr {
//         if self
//             .next_if(|t| t.is_op_a("λ") || t.is_op_a("\\"))
//             .is_some()
//         {
//             let head = Box::new(self.expression(Precedence::Fn));
//             let tail = Box::new(
//                 self.closure()
//                     .and_then(|mut lhs| {
//                         while let Token::Id(..) = self.peek() {
//                             let tail = Box::new(self.expression(Precedence::Fn));
//                             let start = lhs.span();
//                             let end = tail.span();
//                             let span = Span::from((start, end));
//                             lhs = Expr::Closure(Box::new(lhs), tail, span);
//                         }
//                         if self.next_if(|t| t.is_op_a("->")).is_none() {
//                             let span = self.peek().span();
//                             return self.report(Error::Unknown(span));
//                         };
//                         let body = self.closure();
//                         let start = lhs.span();
//                         let end = body.span();
//                         let span = Span::from((start, end));
//                         Expr::Closure(Box::new(lhs), Box::new(body), span)
//                     })
//                     .or_else(|_| {
//                         self.remove_last_error();
//                         if self.next_if(|t| t.is_op_a("->")).is_none() {
//                             let span = self.peek().span();
//                             return self.report(Error::Unknown(span));
//                         };
//                         self.closure()
//                     }),
//             );
//             let start = head.span();
//             let end = tail.span();
//             let span = Span::from((start, end));
//             return Expr::Closure(head, tail, span);
//         }
//         self.conditional()
//     }
//
//     pub(crate) fn conditional(&mut self) -> Expr {
//         let start = self.peek().span();
//         if self.next_if(|t| t.is_keyword_a("if")).is_some() {
//             let condition = self.expression(Precedence::None);
//             if condition.is_error() {
//                 self.remove_last_error();
//                 let end = condition.span();
//                 let span = Span::from((start, end));
//                 return self.report(Error::ExpectedConditionForStatement(span));
//             }
//             if self.next_if(|t| t.is_keyword_a("then")).is_none() {
//                 let span = self.peek().span();
//                 return self.report(Error::Unknown(span));
//             }
//             let branch1 = self.closure();
//             if self.next_if(|t| t.is_keyword_a("else")).is_none() {
//                 let span = self.peek().span();
//                 return self.report(Error::Unknown(span));
//             }
//             let branch2 = self.closure();
//             let end = branch2.span();
//             let span = Span::from((start, end));
//             return Expr::IfElse(
//                 Box::new(condition),
//                 Box::new(branch1),
//                 Box::new(branch2),
//                 span,
//             );
//         }
//         self.call(Precedence::None)
//     }
//
//     pub(crate) fn call(&mut self, min_bp: Precedence) -> Expr {
//         let mut lhs = self.expression(min_bp);
//         if match self.peek() {
//             Token::Op(ref op, ..) if op == "(" => true,
//             Token::Op(ref op, ..) if op == "[" => true,
//             Token::Op(..) | Token::KeyWord(..) | Token::Eof(..) => false,
//             _ => true,
//         } {
//             let mut args = vec![];
//             let last;
//             loop {
//                 match self.peek() {
//                     Token::Op(ref op, ..) if op == "(" || op == "[" => {}
//                     Token::Op(.., span) | Token::Eof(.., span) => {
//                         last = Some(span);
//                         break;
//                     }
//                     _ => {}
//                 };
//                 args.push(self.expression(Precedence::None));
//             }
//             let start = lhs.span();
//             let end = last.unwrap_or(start);
//             let span = Span::from((start, end));
//             lhs = Expr::App(Box::new(lhs), args, span);
//         }
//         lhs
//     }
//
//     fn prefix_op(&mut self, op: &str, span: Span) -> Expr {
//         match op {
//             "(" => {
//                 self.next();
//                 let lhs = self.closure();
//                 if self.next_if(|t| t.is_op_a(")")).is_none() {
//                     let span = self.peek().span();
//                     // return self.report("E13", "closing ')' missing", span);
//                     return self.report(Error::Unknown(span));
//                 };
//                 lhs
//             }
//             "[" => {
//                 self.next();
//                 let mut lhs = vec![];
//                 while !self.peek().is_op_a("]") {
//                     let expr = self.closure();
//                     lhs.push(expr);
//                     self.next_if(|t| t.is_op_a(","));
//                 }
//                 if self.next_if(|t| t.is_op_a("]")).is_none() {
//                     let span = lhs.last().map(|t| t.span()).unwrap_or(self.peek().span());
//                     return self.report(Error::Unknown(span));
//                 };
//                 let span = lhs
//                     .first()
//                     .and_then(|f| {
//                         lhs.last().map_or(Some(f.span()), |l| {
//                             Some(Span::from((f.span(), l.span())))
//                         })
//                     })
//                     .unwrap_or_default();
//                 Expr::Array(lhs, span)
//             }
//             o @ ("-" | "!") => {
//                 self.next();
//                 let op = Op::try_from(o).unwrap();
//                 let lhs = self.expression(Precedence::Unary);
//                 let end = lhs.span();
//                 let span = Span::from((span, end));
//                 Expr::Unary(op, Box::new(lhs), span)
//             }
//             _ => {
//                 let mut l = self.lexer.clone();
//                 l.next();
//                 let is_closing_pran = l.peek().map(|t| t.is_op_a(")")).unwrap_or(false);
//                 if Op::try_from(op).is_ok() && is_closing_pran {
//                     self.lexer = l;
//                     let op = Op::try_from(op).unwrap();
//                     Expr::Atom(Atom::Id(format!("({op})")), span)
//                 } else {
//                     self.report(Error::UnknownOperator(span))
//                 }
//             }
//         }
//     }
//
//     pub(crate) fn expression(&mut self, min_bp: Precedence) -> Expr {
//         let mut lhs = match self.peek() {
//             Token::KeyWord(ref b, _, span) if b == "true" => {
//                 self.next();
//                 Expr::Atom(Atom::Bool(true), span)
//             }
//             Token::KeyWord(ref b, _, span) if b == "false" => {
//                 self.next();
//                 Expr::Atom(Atom::Bool(false), span)
//             }
//             Token::Int(int, _, span) => {
//                 self.next();
//                 Expr::Atom(Atom::Int(int.parse().unwrap()), span)
//             }
//             Token::Float(float, _, span) => {
//                 self.next();
//                 Expr::Atom(Atom::Float(float.parse().unwrap()), span)
//             }
//             Token::Id(ref id, _, span) => {
//                 self.next();
//                 Expr::Atom(Atom::Id(id.into()), span)
//             }
//             Token::String(ref string, _, span) => {
//                 self.next();
//                 Expr::Atom(Atom::String(string.into()), span)
//             }
//             Token::Char(ref c, _, span) => {
//                 self.next();
//                 if c.chars().count() > 1 {
//                     // return self.report("E3", "invalid op char", span);
//                     return self.report(Error::Unknown(span));
//                 }
//                 let Some(c) = c.chars().next() else {
//                     // return self.report("E4", "invalid char definition", span);
//                     return self.report(Error::Unknown(span));
//                 };
//                 Expr::Atom(Atom::Char(c), span)
//             }
//             Token::Op(ref op, _, span) => self.prefix_op(op, span),
//             _ => {
//                 let span = self.peek().span();
//                 // return self.report("E5", "invalid token", span);
//                 return self.report(Error::Unknown(span));
//             }
//         };
//         let start_span = lhs.span();
//         loop {
//             let token = self.peek();
//             if token.position() == &TokenPosition::End
//                 || token.position() == &TokenPosition::FullSpan
//             {
//                 break;
//             }
//             let cbp: Precedence = match token.clone() {
//                 Token::Op(..) => Precedence::try_from(token.clone()).unwrap(),
//                 Token::KeyWord(ref k, ..) if k == "and" => {
//                     Precedence::try_from(token.clone()).unwrap()
//                 }
//                 Token::KeyWord(ref k, ..) if k == "or" => {
//                     Precedence::try_from(token.clone()).unwrap()
//                 }
//                 Token::KeyWord(ref k, ..) if k == "mod" => {
//                     Precedence::try_from(token.clone()).unwrap()
//                 }
//                 _ => break,
//             };
//             if cbp <= min_bp {
//                 break;
//             }
//             let span = Span::from((start_span, token.span()));
//             match cbp {
//                 Precedence::Term
//                 | Precedence::Factor
//                 | Precedence::Comparison
//                 | Precedence::Equality
//                 | Precedence::And
//                 | Precedence::Or => {
//                     let _ = self.next();
//                     let rhs = self.expression(cbp);
//                     lhs = Expr::Binary(
//                         Op::try_from(&token).unwrap(),
//                         Box::new(lhs),
//                         Box::new(rhs),
//                         span,
//                     );
//                 }
//                 Precedence::Assignment | Precedence::None => break,
//                 Precedence::RLPipe => {
//                     let _ = self.next();
//                     let rhs = self.closure();
//                     if let Expr::App(_, args, _) = &mut lhs {
//                         args.push(rhs)
//                     } else {
//                         let l = Box::new(lhs);
//                         lhs = Expr::App(l, vec![rhs], span);
//                     }
//                 }
//                 Precedence::LRPipe => {
//                     let _ = self.next();
//                     let mut rhs = self.closure();
//                     if let Expr::App(_, args, _) = &mut rhs {
//                         args.push(lhs);
//                         lhs = rhs;
//                     } else {
//                         let r = Box::new(rhs);
//                         lhs = Expr::App(r, vec![lhs], span);
//                     }
//                 }
//                 _ => {
//                     // lhs = self.report("E5", "invalid token", span);
//                     return self.report(Error::Unknown(span));
//                 }
//             }
//         }
//         lhs
//     }
// }
