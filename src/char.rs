use crate::parser::{ParseError, ParseResult, Parser};

pub struct Char {
    expected: char,
}

impl Char {
    fn new(expected: char) -> Self {
        Char { expected }
    }
}

impl Parser for Char {
    fn parse(&self, input: &String) -> Result<ParseResult, ParseError> {
        let mut input = input.clone();

        if input.len() == 0 {
            return Err(ParseError {
                message: "Unexpected end of input".to_string(),
            });
        }

        if input.chars().nth(0).unwrap() != self.expected {
            return Err(ParseError {
                message: format!(
                    "Unexpected character {}, expected {}",
                    input.chars().nth(0).unwrap(),
                    self.expected
                ),
            });
        }

        input.remove(0);

        Ok(ParseResult {
            rest: input,
            result: self.expected.into(),
        })
    }

    fn to_string(&self) -> String {
        format!("{}", self.expected)
    }
}

pub fn character(expected: char) -> Char {
    Char::new(expected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let c = character('a');

        assert_eq!(c.to_string(), "/a/".to_string())
    }

    #[test]
    fn parse_single() {
        let c = character('a');

        assert_eq!(
            c.parse(&"a input".to_string()).unwrap().rest,
            " input".to_string()
        );
    }
}
