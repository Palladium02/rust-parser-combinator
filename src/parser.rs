pub trait Parser {
    fn parse(&self, input: &String) -> Result<ParseResult, ParseError>;

    fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub rest: String,
    pub result: String,
}

impl ParseResult {
    fn get_result(&self) -> String {
        self.result.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
}
