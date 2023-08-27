use crate::parser::{ParseError, ParseResult, Parser};

pub struct NOf {
    parser: Box<dyn Parser>,
    n: usize,
}

impl NOf {
    fn new(n: usize, parser: impl Parser + 'static) -> Self {
        NOf {
            n,
            parser: Box::new(parser),
        }
    }
}

impl Parser for NOf {
    fn parse(&self, input: &String) -> Result<ParseResult, ParseError> {
        let input = input.clone();
        let mut i: usize = 0;
        let mut rest = input;
        let mut results: Vec<Result<ParseResult, ParseError>> = Vec::new();

        while i < self.n {
            let result = self.parser.parse(&rest);
            if result.is_err() {
                return result;
            }

            rest = result.clone().unwrap().rest;
            results.push(result);
            i += 1;
        }

        let result = results
            .last()
            .unwrap()
            .clone()
            .unwrap()
            .result
            .repeat(self.n);

        Ok(ParseResult { rest, result })
    }

    fn to_string(&self) -> String {
        format!("({}){{{}}}", self.parser.to_string(), self.n)
    }
}

pub fn n_of(n: usize, parser: impl Parser + 'static) -> NOf {
    NOf::new(n, parser)
}

#[cfg(test)]
mod tests {
    use crate::char::character;

    use super::*;

    #[test]
    fn to_string() {
        let p = n_of(2, character('a'));

        assert_eq!(p.to_string(), "(a){2}".to_string())
    }

    #[test]
    fn parse() {
        let p = n_of(2, character('a'));

        assert_eq!(p.parse(&"aa".to_string()).unwrap().rest, "".to_string())
    }
}
