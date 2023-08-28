use crate::parser::{ParseError, ParseResult, Parser};

pub struct OneOrMore {
    parser: Box<dyn Parser>,
}

impl OneOrMore {
    fn new(parser: Box<dyn Parser + 'static>) -> Self {
        OneOrMore { parser }
    }
}

impl Parser for OneOrMore {
    fn parse(&self, input: &String) -> Result<ParseResult, ParseError> {
        let mut iterations = 0;
        let mut rest = input.clone();
        let mut last_error = "".to_string();

        let mut results: Vec<String> = vec![];
        let mut should_run = true;

        while should_run {
            let result = self.parser.parse(&rest);
            match &result {
                Ok(ok) => {
                    rest = ok.rest.clone();
                    results.push(ok.result.clone());
                    iterations += 1;
                }
                Err(err) => {
                    last_error = err.message.clone();
                    should_run = false;
                }
            }
        }

        if iterations == 0 {
            return Err(ParseError {
                message: last_error,
            });
        }

        Ok(ParseResult {
            rest,
            result: results.join(""),
        })
    }

    fn to_string(&self) -> String {
        format!("({})+", self.parser.to_string())
    }
}

pub fn one_or_more(parser: Box<dyn Parser + 'static>) -> OneOrMore {
    OneOrMore::new(parser)
}

#[cfg(test)]
mod tests {
    use crate::char::character;

    use super::*;

    #[test]
    fn to_string() {
        let p = one_or_more(character('a'));

        assert_eq!(p.to_string(), "(a)+".to_string())
    }

    #[test]
    fn parse_single() {
        let c = one_or_more(character('a'));

        assert_eq!(
            c.parse(&"a input".to_string()).unwrap().rest,
            " input".to_string()
        );

        assert_eq!(
            c.parse(&"aa input".to_string()).unwrap().result,
            "aa".to_string()
        );
    }
}
