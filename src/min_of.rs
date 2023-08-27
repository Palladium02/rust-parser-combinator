use crate::parser::{ParseError, ParseResult, Parser};

pub struct MinOf {
    n: usize,
    parser: Box<dyn Parser>,
}

impl MinOf {
    fn new(n: usize, parser: impl Parser + 'static) -> Self {
        MinOf {
            n,
            parser: Box::new(parser),
        }
    }
}

impl Parser for MinOf {
    fn parse(&self, input: &String) -> Result<ParseResult, ParseError> {
        let mut rest = input.clone();
        let mut iterations: usize = 0;
        let mut results: Vec<String> = Vec::new();

        while rest.len() > 0 {
            let result = self.parser.parse(&rest);
            match &result {
                Ok(ok) => {
                    rest = ok.rest.clone();
                    results.push(ok.result.clone());
                    iterations += 1;
                }
                Err(err) => {
                    if iterations > 0 {
                        break;
                    }
                    return Err(ParseError {
                        message: err.message.clone(),
                    });
                }
            }
        }

        if iterations < self.n {
            return Err(ParseError {
                message: format!(
                    "The parser was only satisfied {} of at least {} times",
                    iterations, self.n
                ),
            });
        }

        Ok(ParseResult {
            rest,
            result: results.join(""),
        })
    }

    fn to_string(&self) -> String {
        format!("{}{{{},}}", self.parser.to_string(), self.n)
    }
}

pub fn min_of(n: usize, parser: impl Parser + 'static) -> MinOf {
    MinOf::new(n, parser)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::char::character;

    #[test]
    fn to_string() {
        let p = min_of(2, character('a'));

        assert_eq!(p.to_string(), "a{2,}".to_string())
    }

    #[test]
    fn parse() {
        let p = min_of(2, character('a'));

        let result = p.parse(&"aa".to_string());
        assert_eq!(result.is_ok(), true)
    }
}
