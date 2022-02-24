use super::token::{Spanned, Token, BlockType};
pub fn _display_indent(tokens: &[Spanned<Token>], src: &str) {
    let mut indent = 0;
    for token in tokens.iter() {
        if let Token::CloseBlock(_) = &token.0 {
            indent -= 4;
        }
        println!(
            "{}{:?} | {:?} | {:?}{}",
            (0..indent).map(|_| " ").collect::<String>(),
            token.0,
            &src[token.1.clone()],
            token.1.clone(),
            match token.0 {
                Token::CloseBlock(BlockType::Fn) => "\n",
                _ => "",
            }
        );
        if let Token::OpenBlock(_) = &token.0 {
            indent += 4;
        }
    }
}

pub fn _display_loc_of_span(tokens: &[Spanned<Token>], src: &str) {
    for token in tokens.iter() {
        println!(
            "{:?} | {:?} | {:?}",
            token.1.clone(),
            token.0,
            &src[token.1.clone()]
        );
    }
}
