use crate::parser::{ParseError, ParseResult, Parser};

pub struct Sequence {
    parsers: Vec<Box<dyn Parser>>,
}

impl Sequence {
    fn new(parsers: Vec<Box<dyn Parser + 'static>>) -> Self {
        Sequence { parsers }
    }
}

impl Parser for Sequence {
    fn parse(&self, input: &String) -> Result<ParseResult, ParseError> {
        let mut results: Vec<String> = Vec::new();
        let input = input.clone();
        let mut acc = input.clone();
        for parser in self.parsers.iter() {
            let result = parser.parse(&acc);
            if result.is_err() {
                return Err(ParseError {
                    message: result.unwrap_err().message,
                });
            }

            let unwrapped = result.unwrap();
            results.push(unwrapped.result);
            acc = unwrapped.rest;
        }

        Ok(ParseResult {
            rest: acc,
            result: results.join(""),
        })
    }

    fn to_string(&self) -> String {
        let mut string = "".to_string();
        for parser in self.parsers.iter() {
            string.push_str(parser.to_string().as_str());
        }
        string
    }
}

pub fn sequence(parsers: Vec<Box<dyn Parser>>) -> Sequence {
    Sequence::new(parsers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::char::character;

    #[test]
    fn to_string() {
        let p = sequence(vec![Box::new(character('a')), Box::new(character('b'))]);

        assert_eq!(p.to_string(), "ab".to_string())
    }

    #[test]
    fn parse() {
        let p = sequence(vec![Box::new(character('a')), Box::new(character('b'))]);

        let result = p.parse(&"ab".to_string());

        assert_eq!(result.is_ok(), true)
    }
}
