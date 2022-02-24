use super::token::{BlockType, KeyWord, Span, Spanned, Token};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    pos: Span,
    stream: Peekable<Chars<'a>>,
    indent_stack: Vec<u16>,
    block_stack: Vec<BlockType>,
    id_count: usize,
    pub tokens: Vec<Spanned<Token>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            pos: 0..1,
            stream: src.chars().peekable(),
            indent_stack: Vec::new(),
            block_stack: Vec::new(),
            id_count: 0,
            tokens: Vec::new(),
        }
    }
    pub fn parse(mut self) -> Self {
        while let Some(c) = self.stream.next() {
            match c {
                '{' => self.block_comment(),
                '-' if self.stream.peek() == Some(&&'-') => self.line_comment(),
                '\'' => self.character(),
                '/' => self.add_token(Token::Op("/".into())),
                '!' if self.stream.peek() == Some(&&'=') => self.add_token(Token::Op("!=".into())),
                '!' => self.add_token(Token::Op("!".into())),
                '<' if self.stream.peek() == Some(&&'=') => self.add_token(Token::Op("<=".into())),
                '<' => self.add_token(Token::Op("<".into())),
                '>' if self.stream.peek() == Some(&&'=') => self.add_token(Token::Op(">=".into())),
                '>' => self.add_token(Token::Op(">".into())),
                '=' if self.stream.peek() == Some(&&'=') => self.add_token(Token::Op("==".into())),
                '=' if self.block_stack.last() != Some(&BlockType::Let) => self.open_block(),
                '=' => self.add_token(Token::Op("=".into())),
                '+' => self.add_token(Token::Op("+".into())),
                '-' if self.stream.peek().map_or(false, |c| c.is_numeric()) => self.number(c),
                '-' => self.add_token(Token::Op("-".into())),
                '*' => self.add_token(Token::Op("*".into())),
                '(' => {
                    self.block_stack.push(BlockType::Paren);
                    self.add_token(Token::Ctrl('('));
                }
                ')' => {
                    self.close_arg_block();
                    self.block_stack.pop();
                    self.add_token(Token::Ctrl(')'));
                }
                '[' => self.add_token(Token::Ctrl('[')),
                ']' => self.add_token(Token::Ctrl(']')),
                ',' => self.add_token(Token::Ctrl(',')),
                '.' => self.add_token(Token::Ctrl('.')),
                ';' => self.add_token(Token::Ctrl(';')),
                '"' => self.string(),
                ' ' => self.new_span(),
                '\n' if self.stream.peek() == Some(&&' ') => {
                    self.close_arg_block();
                    self.end_expr();
                    self.whitespace();
                },
                '\n' if self.stream.peek().unwrap_or(&'\0').is_ascii_alphabetic() => {
                    self.close_arg_block();
                    self.end_expr();
                    self.close_block()
                }
                '\n' => {
                    self.close_arg_block();
                    self.end_expr();
                    self.new_span();
                }
                c if c.is_numeric() => self.number(c),
                c if c.is_ascii_alphabetic() => self.identifier(c),
                _ => self.error(c),
            }
            self.check_open_arg_block();
        }
        for block in self.block_stack.iter().rev() {
            self.tokens
                .push((Token::CloseBlock(*block), self.pos.start..self.pos.start));
        }
        // self.tokens
        //     .push((Token::Eof, self.pos.start..self.pos.start));
        self
    }

    fn advance(&mut self) {
        self.pos.end += 1;
    }

    fn new_span(&mut self) {
        self.pos.start = self.pos.end;
        self.advance();
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push((token, self.pos.clone()));
        self.new_span();
    }

    fn error(&mut self, c: char) {
        self.tokens.push((
            Token::Error(format!("Unknown Char: {}", c)),
            self.pos.clone(),
        ));
        self.new_span();
    }

    fn line_comment(&mut self) {
        while let Some(c) = self.stream.next_if(|&c| c != '\n') {
            self.advance();
        }
        self.new_span();
    }

    fn block_comment(&mut self) {
        let mut last = '\0';
        while let Some(c) = self.stream.next_if(|&c| c != '}' && last != '-') {
            last = c;
            self.advance();
        }
        self.new_span();
    }

    fn whitespace(&mut self) {
        if let Some(BlockType::Let) = self.block_stack.last() {
            self.tokens
                .push((Token::CloseBlock(BlockType::Let), self.pos.clone()));
            let _ = self.block_stack.pop();
        }
        let mut indent_depth = 0;
        while let Some(_) = self.stream.next_if(|&c| c == ' ') {
            indent_depth += 1;
            self.advance();
        }
        self.indent_stack.push(indent_depth);
        self.new_span();
    }

    fn character(&mut self) {
        // This need to be fixed to handle if a character is bad.
        if let Some(c) = self.stream.next() {
            self.advance();
            let _ = self.stream.next();
            self.advance();
            self.tokens.push((Token::Char(c), self.pos.clone()));
            self.new_span();
        }
    }

    fn number(&mut self, c: char) {
        let mut number = c.to_string();
        while let Some(c) = self
            .stream
            .next_if(|&c| c.is_numeric() || (c == '.' && !number.contains('.')))
        {
            number.push(c);
            self.advance();
        }
        let token = if number.contains('.') {
            Token::Float(number)
        } else {
            Token::Int(number)
        };
        self.tokens.push((token, self.pos.clone()));
        self.new_span();
    }

    fn identifier(&mut self, c: char) {
        self.id_count = if self.block_stack.last() == Some(&BlockType::Pram) {
            self.id_count + 1
        } else {
            self.id_count
        };
        let mut idt = c.to_string();
        while let Some(c) = self.stream.next_if(|&c| c.is_ascii_alphanumeric()) {
            idt.push(c);
            self.advance();
        }
        let token_id = KeyWord::lookup(&idt).map_or(Token::Id(idt), Token::KeyWord);
        let mut token_id = self.pram(token_id);
        if let Token::KeyWord(KeyWord::Do) = token_id.0 {
            self.block_stack.push(BlockType::Do);
            token_id = (Token::OpenBlock(BlockType::Do), token_id.1);
        }
        if let Token::KeyWord(KeyWord::Let) = token_id.0 {
            self.block_stack.push(BlockType::Let);
            token_id = (Token::OpenBlock(BlockType::Let), token_id.1);
        }
        self.tokens.push(token_id);
        self.args();
        self.new_span();
    }

    fn args(&mut self) {
        match self.block_stack.last() {
            Some(&BlockType::FnBlock) | Some(&BlockType::Paren) | Some(&BlockType::Do) => {
                self.block_stack.push(BlockType::Arg);
                self.tokens
                    .push((Token::OpenBlock(BlockType::Arg), self.pos.clone()));
            }
            _ => {},
        }
    }

    fn pram(&mut self, token: Token) -> Spanned<Token> {
        use Token::*;
        let last_token = self.tokens.last().map(Clone::clone);
        match last_token {
            Some((Id(_), _)) if self.block_stack.last() != Some(&BlockType::Pram) => {
                self.id_count += 2;
                self.tokens
                    .push((Token::OpenBlock(BlockType::Pram), self.pos.clone()));
                self.block_stack.push(BlockType::Pram);
                (token, self.pos.clone())
            }
            _ => (token, self.pos.clone()),
        }
    }

    fn string(&mut self) {
        let mut string = String::new();
        while let Some(c) = self.stream.next() {
            self.advance();
            if c == '"' {
                break;
            } else {
                string.push(c);
            }
        }
        self.tokens.push((Token::String(string), self.pos.clone()));
        let _ = self.stream.next();
        self.advance();
        self.new_span();
    }

    fn open_block(&mut self) {
        if let Some(BlockType::Pram) = self.block_stack.last() {
            let _ = self.block_stack.pop();
            self.id_count += 1;
            self.tokens
                .push((Token::CloseBlock(BlockType::Pram), self.pos.clone()));
        }
        // TODO: Remove the unwrap.
        let pos = self.tokens.last().unwrap().1.clone();
        self.block_stack.push(BlockType::Fn);
        self.tokens.insert(
            self.tokens.len().saturating_sub(self.id_count+1),
            (Token::OpenBlock(BlockType::Fn), pos),
        );
        self.id_count = 0;
        self.block_stack.push(BlockType::FnBlock);
        self.tokens.push((Token::Op("=".into()), self.pos.clone()));
        self.tokens
            .push((Token::OpenBlock(BlockType::FnBlock), self.pos.clone()));
        self.new_span();
    }

    fn close_block(&mut self) {
        match self.block_stack.pop() {
            Some(BlockType::Fn) => {
                self.tokens.push((Token::CloseBlock(BlockType::Fn), self.pos.clone()));
                self.new_span();
            }
            Some(block) => {
                self.tokens
                    .push((Token::CloseBlock(block), self.pos.clone()));
                self.close_block();
            }
            _ => {}
        }
    }

    fn close_arg_block(&mut self) {
        if let Some(BlockType::Arg) = self.block_stack.last() {
            let _ = self.block_stack.pop();
            if let Some((Token::OpenBlock(BlockType::Arg), _)) = self.tokens.last() {
                let _ = self.tokens.pop();
            } else {
                self.tokens.push((Token::CloseBlock(BlockType::Arg), self.pos.clone()));
            }
        }
    }

    fn check_open_arg_block(&mut self) {
        if let (Some(bs), Some((last, _)), Some((sec_last, _))) = (self.block_stack.last(), self.tokens.last(), self.tokens.get(self.tokens.len() - 2)) {
            if bs == &BlockType::Arg {
                match last {
                    Token::Op(_) if sec_last == &Token::OpenBlock(BlockType::Arg) => {
                    self.tokens.remove(self.tokens.len() - 2);
                    self.block_stack.pop();
                    }
                    _ => {}
                }
            }
        }
    }

    fn end_expr(&mut self) {
        use BlockType::*;
        if let (Some(bs), Some((lt, _))) = (self.block_stack.last(), self.tokens.last()) {
            match bs {
                Do if lt != &Token::OpenBlock(BlockType::Do) => self.tokens.push((Token::Ctrl(';'), self.pos.clone())),
                _ => {},
            }
        }
    }
}
