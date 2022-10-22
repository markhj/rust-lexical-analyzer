use crate::langdef::LanguageDefinition;
use crate::tokenizer::{
    Context::*,
    TokenType::*,
};
use std::{
    ops::Add,
    str::Chars,
};
use regex::Regex;

/// # TokenStream
/// A list/stream container of ``Token`` based on ``Vec<Token>``
pub type TokenStream = Vec<Token>;

/// # Tokenizer struct
/// The main Tokenizer instance which contains the static
/// methods to be called to retrieve a ``TokenStream``
#[derive(Debug, PartialEq, Clone)]
pub struct Tokenizer;

/// # Token struct
///
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    /// Keywords are recognized words found in the source code, and outside
    /// contexts such as quotes, docblocks and comments.
    /// Classic examples include ``if``, ``else``, ``match``, ``switch``, ``public``, and so on.
    /// Keywords are specified in the ``langdef``
    Keyword(String),

    /// Identifiers are words found in the code which couldn't be mapped to a
    /// keyword. They are typically names of variables, constants and functions.
    Identifier(String),

    /// Punctuators are structures in the code such as parentheses, brackets and statement endings.
    /// For example: {} () and ;
    Punctuator(char),

    /// Operators are typically arithmetic functions such as addition (+), subtraction (-),
    /// multplication (-), assignment (=) and comparison (==)
    Operator(String),

    /// A literal can be a string, number, boolean or ``null``.
    Literal(String),
}

#[derive(Debug, PartialEq)]
enum Context {
    Quotes,
    Comment,
    DocBlock,
}

impl Tokenizer {
    /// # Tokenize
    /// Build a ``TokenStream`` based on the passed document and ``LanguageDefinition``.
    /// The function iterates over the document character by character, and slices the
    /// content into tokens such as keywords, literals and punctuators.
    ///
    /// A Result will be returned containing either an error (typically based on syntax)
    /// or a ``TokenStream`` which is essentially an ordered ``Vec`` containing the tokens.
    ///
    /// Comments and docblocks are ignored.
    pub fn tokenize(
        langdef: &LanguageDefinition,
        document: String,
    ) -> Result<TokenStream, ()> {
        let mut stream: TokenStream = TokenStream::new();

        // This variable is to remember if we have entered a certain context,
        // for example inside quoted strings or comments, which require different
        // handling than other scenarios
        let mut context: Option<Context> = None;

        // The buffer holds none, one or several characters, which are picked up,
        // until we figure out what to do with them
        let mut buffer: String = String::new();

        // Shorthand to determine if we have encountered the end of the line
        let mut is_eol: bool;

        // Look at the next (peek) and previous characters in the document
        let mut peek: char;
        let mut prev: Option<char> = None;

        // Regular iterator/counter to define at which index we are in
        // the complete document
        let mut i: usize = 0;

        // The entire document separated into single characters
        let chars: Chars = document.chars();

        for e in document.chars() {
            is_eol = e == '\n' || e == '\r';

            // Store the next character ("peek") for analysis
            peek = chars.clone().nth(i + 1).unwrap_or(' ');

            // When there's no context and the current and next character form /*
            // we're entering a docblock
            if context.is_none() && e == '/' && peek == '*' {
                context = Some(DocBlock);

            // If we are in docblock context and encounter */, which indicates the end
            // of a docblock, we leave that context here.
            // Since docblocks should be ignored, we will not do anything with
            // eventual buffer content
            } else if context.is_some() && prev.is_some() && context.as_ref().unwrap() == &DocBlock && e == '/' && prev.unwrap() == '*' {
                context = None;

            // If we aren't in an established context, but encounter //, we will
            // enter a comment context (which is also just to be ignored)
            } else if context.is_none() && e == '/' && peek == '/' {
                context = Some(Comment);

            // When we are inside a quote context, we want to add the character to the
            // the buffer, unless it's a quote, in which case we leave quote context
            // @todo: Implement escaping of quotes (using prev variable)
            } else if context.is_some() && context.as_ref().unwrap() == &Quotes {
                Self::context_quotes(e, &mut stream, &mut buffer, &mut context);

            // We ignore docblock context, therefore no actions are taken, besides
            // making sure we enter a scope with no actions defined
            } else if context.is_some() && context.as_ref().unwrap() == &DocBlock {

            // Ordinary comments (// and #) are terminated when encountering the end of the line
            } else if context.is_some() && context.as_ref().unwrap() == &Comment && is_eol {
                context = None;

            // When there's no defined context, we will use a match pattern to decide what
            // should happen, based on which character we've seen
            } else if context.is_none() {
                Self::context_none(&langdef, e, &mut stream, &mut buffer, &mut context);
            }

            prev = Some(e);
            i = i + 1;
        }

        // There can still be residue in the buffer, if we haven't encountered a condition
        // which triggers adding to the buffer. We deal with that here:
        if !buffer.is_empty() {
            Self::add_to_stream(&langdef, &mut stream, &mut buffer);
        }

        Ok(stream)
    }

    /// # Context none
    /// Determine what should happen with the stream and context
    /// based on the character at the pointer
    fn context_none(
        langdef: &LanguageDefinition,
        e: char,
        stream: &mut TokenStream,
        buffer: &mut String,
        context: &mut Option<Context>,
    ) {
        match e {
            // Punctuators:
            ';' | '{' | '}' | '(' | ')' | '[' | ']' => {
                Self::add_to_stream(&langdef, stream, buffer);
                stream.insert(stream.len(), Token {
                    token_type: Punctuator(e),
                });
            },

            // Operators:
            '+' | '-' | '/' | '*' | '%' => {
                Self::add_to_stream(&langdef, stream, buffer);
                stream.insert(stream.len(), Token {
                    token_type: Operator(e.to_string()),
                });
            },

            // Comment
            '#' => {
                *context = Some(Comment);
            },

            // Enter Quote context
            '"' => {
                *context = Some(Quotes);
            },

            // Space or end of line
            ' ' | '\n' | '\r' => {
                Self::add_to_stream(&langdef, stream, buffer);
            },

            // In all other cases, add the character to the buffer, and take
            // no additional actions
            _ => {
                *buffer = buffer.to_string().add(e.to_string().as_str());
            },
        }
    }

    /// # Context quotes
    /// Helper function for when the cursor is between two
    /// quotation marks (")
    fn context_quotes(
        char: char,
        stream: &mut TokenStream,
        buffer: &mut String,
        context: &mut Option<Context>,
    ) {
        match char {
            '"' => {
                stream.insert(stream.len(), Token {
                    token_type: Literal(buffer.clone()),
                });
                *buffer = String::new();
                *context = None;
            },
            _ => {
                *buffer = buffer.to_string().add(char.to_string().as_str());
            }
        }
    }

    /// # Add to stream
    /// Helper function to streamline the actions taken, when we want
    /// add the contents of the buffer to the Token Stream
    fn add_to_stream(
        langdef: &LanguageDefinition,
        stream: &mut TokenStream,
        buffer: &mut String,
    ) {
        let output: Option<TokenType> = Self::parse_token_type(&langdef, &buffer);
        if output.is_some() {
            stream.insert(stream.len(), Token {
                token_type: output.unwrap(),
            });
        }
        *buffer = String::new();
    }

    /// # Parse token type
    /// Based on the looks of the buffer content, we will return
    /// a ``TokenType`` enum. Examples include ``Literal``, ``Operator``
    /// and ``Identifier``.
    fn parse_token_type(
        langdef: &LanguageDefinition,
        buffer: &String,
    ) -> Option<TokenType> {
        let regex_literal: Regex = Regex::new(r"^([0-9]+(\.[0-9]+)?|true|false|null)$").unwrap();

        if regex_literal.is_match(buffer.trim()) {
            return Some(Literal(buffer.clone()));
        }

        if langdef.has_keyword(buffer) {
            return Some(Keyword(buffer.clone()));
        }

        if buffer == "=" || buffer == "==" {
            return Some(Operator(buffer.to_string()));
        }

        if buffer.trim().is_empty() {
            return None;
        }

        Some(Identifier(buffer.clone()))
    }
}
