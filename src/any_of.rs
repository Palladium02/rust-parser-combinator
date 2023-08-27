use crate::parser::{ParseError, ParseResult, Parser};

pub struct AnyOf {
    parsers: Vec<Box<dyn Parser>>,
}

impl AnyOf {
    fn new(parsers: Vec<Box<dyn Parser + 'static>>) -> Self {
        AnyOf { parsers }
    }
}

impl Parser for AnyOf {
    fn parse(&self, input: &String) -> Result<ParseResult, ParseError> {
        let input = input.clone();
        for parser in self.parsers.iter() {
            let result = parser.parse(&input);
            if result.is_ok() {
                return Ok(result.unwrap());
            }
        }
        Err(ParseError {
            message: "None of the given parsers could match the input.".to_string(),
        })
    }

    fn to_string(&self) -> String {
        self.parsers
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join("|")
    }
}

fn any_of(parsers: Vec<Box<dyn Parser + 'static>>) -> AnyOf {
    AnyOf { parsers }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::char::character;

    #[test]
    fn to_string() {
        let p = any_of(vec![Box::new(character('a')), Box::new(character('b'))]);
        assert_eq!(p.to_string(), "a|b".to_string())
    }

    #[test]
    fn parse() {
        let p = any_of(vec![Box::new(character('a')), Box::new(character('b'))]);

        let r1 = p.parse(&"a".to_string());
        assert_eq!(r1.is_ok(), true);

        let r2 = p.parse(&"b".to_string());
        assert_eq!(r2.is_ok(), true);
    }
}
