mod common;

use rust_lexical_analyzer::{
    tokenizer::{
        Tokenizer,
        TokenType::*,
        TokenStream,
    },
    langdef::LanguageDefinition,
};

use common::*;

#[test]
fn basic_syntax() {
    let langdef: LanguageDefinition = default_langdef();
    let result: Result<TokenStream, ()> = Tokenizer::tokenize(
        &langdef,
        get_test_file("basic.txt"),
    );

    test_stream(
        result.as_ref().unwrap(),
        vec![
            Keyword(String::from("let")),
            Identifier(String::from("name")),
            Operator(String::from("=")),
            Literal(String::from("John Doe")),
            Punctuator(';'),
            Keyword(String::from("if")),
            Identifier(String::from("variable")),
            Operator(String::from("==")),
            Literal(String::from("30")),
            Operator(String::from("+")),
            Literal(String::from("18")),
            Punctuator('{'),
            Identifier(String::from("print")),
            Literal(String::from("Hello world")),
            Punctuator(';'),
            Punctuator('}'),
        ],
    );
}

#[test]
fn operators() {
    let langdef: LanguageDefinition = default_langdef();
    let result: Result<TokenStream, ()> = Tokenizer::tokenize(
        &langdef,
        "100 + 100 - 10 * 2 / 2 % 1".to_string(),
    );

    test_stream(
        result.as_ref().unwrap(),
        vec![
            Literal(String::from("100")),
            Operator(String::from("+")),
            Literal(String::from("100")),
            Operator(String::from("-")),
            Literal(String::from("10")),
            Operator(String::from("*")),
            Literal(String::from("2")),
            Operator(String::from("/")),
            Literal(String::from("2")),
            Operator(String::from("%")),
            Literal(String::from("1")),
        ],
    );
}

#[test]
fn comments() {
    let langdef: LanguageDefinition = default_langdef();
    let result: Result<TokenStream, ()> = Tokenizer::tokenize(
        &langdef,
        get_test_file("comments.txt"),
    );

    test_stream(
        result.as_ref().unwrap(),
        vec![
            Keyword(String::from("let")),
            Identifier(String::from("below_docblock")),
            Operator(String::from("=")),
            Literal(String::from("1")),
            Punctuator(';'),

            Keyword(String::from("let")),
            Identifier(String::from("x")),
            Operator(String::from("=")),
            Literal(String::from("2")),
            Punctuator(';'),

            Keyword(String::from("let")),
            Identifier(String::from("x")),
            Operator(String::from("=")),
            Literal(String::from("3")),
            Punctuator(';'),
        ],
    );
}
