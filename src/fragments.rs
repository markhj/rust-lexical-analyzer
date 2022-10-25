use std::ops::Add;
use crate::{
    composer::{ComposerContext, Composition},
    tokenizer::{Token, TokenStream, TokenType::*},
};

/// # Fragment trait
/// Any fragment which can be found and broken down to in the source code,
/// must implement this ``Fragment`` trait
pub trait Fragment {
    fn get_type(&self) -> &'static str;
    fn debug(&self) -> String;
    fn get_context(&self) -> Option<ComposerContext>;
    fn get_token(&self) -> Option<Token>;
    fn get_tokens(&self) -> TokenStream;
    fn get_fragments(&self) -> Option<Composition>;
}

/// # Statement (fragment)
/// Consists of 2+ tokens, for example:
///
/// - ``let x = 1``
/// - ``if x == 2``
///
/// Statements don't contain encapsulations, as these must be evaluated
/// at another time. Therefore:
///
/// ``if (x == 2)`` will not be considered a single statement (because of the parenthesis),
/// but instead on the form ``if [encapsulation]``, and then the encapsulation contains
/// a statement, ``x == 2``, which can be more conveniently evaluated (first)
#[derive(Debug)]
pub struct Statement {
    pub token_stream: TokenStream,
}

impl Fragment for Statement {
    fn get_type(&self) -> &'static str {
        "statement"
    }

    // @todo: Room for improvement in this code
    fn debug(&self) -> String {
        let mut str = String::new();
        for token in self.get_tokens() {
            if !str.is_empty() {
                str = str.add( " ");
            }
            match token.token_type {
                Literal(a) => str = str.add(format!("Literal({})", a).as_str()),
                Operator(a) | Keyword(a) => str = str.add(format!("{}", a).as_str()),
                Identifier(a) => str = str.add(format!("Identifier{}", a).as_str()),
                _ => str = str.add(format!("{:?}", token).as_str()),
            }
        }
        format!("Statement: {}", str)
    }
    fn get_context(&self) -> Option<ComposerContext> {
        None
    }
    fn get_token(&self) -> Option<Token> {
        None
    }
    fn get_tokens(&self) -> TokenStream {
        self.token_stream.clone()
    }
    fn get_fragments(&self) -> Option<Composition> {
        None
    }
}

/// # Single token (fragment)
/// Contains a lone token which exists between statements and encapsulations.
/// This is for example commonly seen with ``else`` keywords.
#[derive(Debug)]
pub struct SingleToken {
    pub token: Token,
}

impl Fragment for SingleToken {
    fn get_type(&self) -> &'static str {
        "single_token"
    }
    fn debug(&self) -> String {
        format!("{:?}", self.token.token_type)
    }
    fn get_context(&self) -> Option<ComposerContext> {
        None
    }
    fn get_token(&self) -> Option<Token> {
        Some(self.token.clone())
    }
    fn get_tokens(&self) -> TokenStream {
        TokenStream::new()
    }
    fn get_fragments(&self) -> Option<Composition> {
        None
    }
}

/// # Encapsulation (fragment)
/// Indicates a body of multiple statements, single tokens and other
/// encapsulations contained within a bracket such as (), [] or {}.
#[derive(Debug)]
pub struct Encapsulation {
    pub context: ComposerContext,
    pub composition: Composition,
}

impl Fragment for Encapsulation {
    fn get_type(&self) -> &'static str {
        "block"
    }
    fn debug(&self) -> String {
        format!("Encapsulation: {:?}", self.composition)
    }
    fn get_context(&self) -> Option<ComposerContext> {
        Some(self.context.clone())
    }
    fn get_token(&self) -> Option<Token> {
        None
    }
    fn get_tokens(&self) -> TokenStream {
        TokenStream::new()
    }
    fn get_fragments(&self) -> Option<Composition> {
        Some(self.composition.clone())
    }
}
