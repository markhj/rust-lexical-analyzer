use crate::tokenizer::TokenType::{self, *};

/// # Language Definition
/// The struct describing the rules and looks of the language
/// to be tokenized and interpreted
#[derive(Debug, Clone, PartialEq)]
pub struct LanguageDefinition {
    keywords: Vec<&'static str>,
    statement_terminator: TokenType,
    block_opener: TokenType,
    block_closer: TokenType,
}

impl LanguageDefinition {
    /// # New language definition
    /// Create a ``LanguageDefinition`` struct
    pub fn new(
        keywords: Vec<&'static str>,
    ) -> LanguageDefinition {
        LanguageDefinition {
            keywords,
            statement_terminator: Punctuator(';'),
            block_opener: Punctuator('{'),
            block_closer: Punctuator('}'),
        }
    }

    /// # Has keyword
    /// Returns true, if the ``keyword`` parameter is defined
    /// as a keyword in the language definition
    pub fn has_keyword(&self, keyword: &String) -> bool {
        self.keywords.contains(&&**keyword)
    }
}
