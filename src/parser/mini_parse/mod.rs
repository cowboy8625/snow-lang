mod combinators;
use super::{Spanned, Token};
pub use combinators::{
    either, left, one_or_more, pair, pred, right, surround, zero_or_more, zero_or_one, ParseResult,
    Parser,
};

#[macro_export]
macro_rules! one_of {
    ($single:expr $(,)?) => {
        $single
    };
    ($first:expr, $($rest:expr),* $(,)?) => {
        either($first, one_of!( $($rest),* ) )
    }
}
