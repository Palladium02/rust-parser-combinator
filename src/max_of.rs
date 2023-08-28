use crate::parser::{ParseError, ParseResult, Parser};

pub struct MaxOf {
    n: usize,
    parser: Box<dyn Parser>,
}

impl MaxOf {
    fn new(n: usize, parser: Box<dyn Parser + 'static>) -> Self {
        MaxOf { n, parser }
    }
}

impl Parser for MaxOf {
    fn parse(&self, input: &String) -> Result<ParseResult, ParseError> {
        let mut rest = input.clone();
        let mut results: Vec<String> = Vec::new();

        for _ in 0..self.n {
            let result = self.parser.parse(&rest);
            if result.is_err() {
                break;
            }

            let unwrapped = result.unwrap();
            rest = unwrapped.rest.clone();
            results.push(unwrapped.result.clone());
        }

        Ok(ParseResult {
            rest,
            result: results.join(""),
        })
    }

    fn to_string(&self) -> String {
        format!("{}{{,{}}}", self.parser.to_string(), self.n)
    }
}

pub fn max_of(n: usize, parser: Box<dyn Parser + 'static>) -> MaxOf {
    MaxOf::new(n, parser)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::char::character;

    #[test]
    fn to_string() {
        let p = max_of(2, character('a'));

        assert_eq!(p.to_string(), "a{,2}".to_string())
    }

    #[test]
    fn parse() {
        let p = max_of(2, character('a'));

        let result = p.parse(&"aa".to_string());
        assert_eq!(result.is_ok(), true)
    }

    #[test]
    fn parse_empty() {
        let p = max_of(2, character('a'));

        let result = p.parse(&"".to_string());
        assert_eq!(result.is_ok(), true)
    }
}
