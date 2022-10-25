mod common;

use rust_lexical_analyzer::{
    tokenizer::{
        Tokenizer,
        TokenType::*,
        TokenStream,
    },
    langdef::LanguageDefinition,
    composer::{
        Composer,
        Composition,
    },
};

use common::*;

#[test]
fn composer() {
    let langdef: LanguageDefinition = default_langdef();
    let result: Result<TokenStream, ()> = Tokenizer::tokenize(
        &langdef,
        get_test_file("composer_std.txt"),
    );

    let composed: Composition = Composer::compose(&langdef, &result.unwrap());
    let block1: Composition = composed.get(1).unwrap().get_fragments().unwrap();

    test_statement(composed.get(0).unwrap(), vec![
        Keyword(String::from("if")),
        Identifier(String::from("x")),
        Operator(String::from("==")),
        Literal(String::from("30")),
    ]);

    test_statement(&block1.get(0).unwrap(), vec![
        Keyword(String::from("if")),
        Literal(String::from("true")),
    ]);

    test_statement(&block1.get(1).unwrap().get_fragments().unwrap().get(0).unwrap(), vec![
        Identifier(String::from("print")),
        Literal(String::from("Hello world")),
    ]);

    assert_eq!(
        Keyword(String::from("else")),
        block1.get(2).unwrap().get_token().unwrap().token_type,
    );

    test_statement(&block1.get(3).unwrap().get_fragments().unwrap().get(0).unwrap(), vec![
        Identifier(String::from("print")),
        Literal(String::from("nested else")),
    ]);

    assert_eq!(
        Keyword(String::from("else")),
        composed.get(2).unwrap().get_token().unwrap().token_type,
    );

    test_statement(&composed.get(3).unwrap().get_fragments().unwrap().get(0).unwrap(), vec![
        Identifier(String::from("print")),
        Literal(String::from("else")),
    ]);
}

#[test]
fn composer_brackets() {
    let expressions = vec![
        "1 + (2 + (3 * 3));".to_string(),
        "1 + (2 + [3 * 3]);".to_string(),
        "1 + [2 + [3 * 3]];".to_string(),
    ];

    for expr in expressions {
        let langdef: LanguageDefinition = default_langdef();
        let result: Result<TokenStream, ()> = Tokenizer::tokenize(&langdef, expr);
        let composed: Composition = Composer::compose(&langdef, &result.unwrap());
        let b1_fragments: Composition = composed.get(1).unwrap().get_fragments().unwrap();

        test_statement(composed.get(0).unwrap(), vec![
            Literal(String::from("1")),
            Operator(String::from("+")),
        ]);

        test_statement(b1_fragments.get(0).unwrap(), vec![
            Literal(String::from("2")),
            Operator(String::from("+")),
        ]);

        test_statement(b1_fragments.get(1).unwrap().get_fragments().unwrap().get(0).unwrap(), vec![
            Literal(String::from("3")),
            Operator(String::from("*")),
            Literal(String::from("3")),
        ]);
    }
}
