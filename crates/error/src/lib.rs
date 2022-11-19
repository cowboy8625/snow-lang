pub type CResult<T> = Result<T, Box<dyn std::error::Error>>;

// enum CResult<S, P> {
//     Scanner(T),
//     Parser(P),
// }

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
