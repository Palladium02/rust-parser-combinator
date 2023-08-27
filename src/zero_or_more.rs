use crate::parser::{ParseError, ParseResult, Parser};

pub struct ZeroOrMore {
    parser: Box<dyn Parser>,
}

impl ZeroOrMore {
    fn new(parser: impl Parser + 'static) -> Self {
        ZeroOrMore {
            parser: Box::new(parser),
        }
    }
}

impl Parser for ZeroOrMore {
    fn parse(&self, input: &String) -> Result<ParseResult, ParseError> {
        let mut rest = input.clone();

        while rest != "".to_string() {
            let result = self.parser.parse(&rest);
            if result.is_err() {
                return Ok(ParseResult {
                    rest: input
                        .chars()
                        .take(input.len() - rest.len())
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join(""),
                    result: "".to_string(),
                });
            }

            rest = result.unwrap().rest;
        }

        Ok(ParseResult {
            rest: "".to_string(),
            result: input.clone(),
        })
    }

    fn to_string(&self) -> String {
        format!("({})*", self.parser.to_string())
    }
}

pub fn zero_or_more(parser: impl Parser + 'static) -> ZeroOrMore {
    ZeroOrMore::new(parser)
}

#[cfg(test)]
mod tests {
    use crate::{char::character, parser::Parser};

    use super::zero_or_more;

    #[test]
    fn to_string() {
        let p = zero_or_more(character('a'));

        assert_eq!(p.to_string(), "(a)*".to_string())
    }

    #[test]
    fn parse_empty() {
        let p = zero_or_more(character('a'));

        let result = p.parse(&"".to_string());
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn parse() {
        let p = zero_or_more(character('a'));

        let result = p.parse(&"aa".to_string());
        assert_eq!(result.is_ok(), true);
    }
}
