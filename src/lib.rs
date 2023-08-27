mod any_of;
mod char;
mod n_of;
mod one_or_more;
mod parser;
mod sequence;
mod zero_or_more;

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
