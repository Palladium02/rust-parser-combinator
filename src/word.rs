use crate::{char::character, parser::Parser, sequence::sequence};

pub fn word(word: String) -> impl Parser {
    sequence(word.chars().map(|c| character(c)).collect::<Vec<_>>())
}
