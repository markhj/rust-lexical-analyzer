#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;
use rust_lexical_analyzer::fragments::Fragment;
use rust_lexical_analyzer::langdef::LanguageDefinition;
use rust_lexical_analyzer::tokenizer::{TokenStream, TokenType};

pub fn test_stream(
    stream: &TokenStream,
    assertions: Vec<TokenType>,
) {
    for i in 0..assertions.len() {
        if i == assertions.len() {
            break;
        }

        assert_eq!(
            assertions.get(i).unwrap().clone(),
            stream.get(i).unwrap().token_type,
        );
    }
}

pub fn test_statement(
    statement: &Box<dyn Fragment>,
    assertions: Vec<TokenType>,
) {
    for i in 0..assertions.len() {
        assert_eq!(
            assertions.get(i).unwrap(),
            &statement.get_tokens().get(i as usize).unwrap().token_type
        );
    }
}

pub fn get_test_file(filename: &str) -> String {
    let frm: String = format!("./tests/res/{}", filename);
    let path: &Path = Path::new(frm.as_str());
    fs::read_to_string(path).expect("Unable to read file")
}

pub fn default_langdef() -> LanguageDefinition {
    LanguageDefinition::new(
        vec!["if", "match", "else", "let"],
    )
}
