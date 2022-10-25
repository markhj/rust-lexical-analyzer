use crate::{
    langdef::LanguageDefinition,
    composer::{
        ComposerContext::*,
        BracketType::*,
    },
    tokenizer::{
        TokenStream,
        TokenType::{self, *},
    },
    fragments::{
        Encapsulation,
        Fragment,
        SingleToken,
        Statement,
    },
};

use std::fmt::Result;
use core::fmt::{Debug, Formatter};

/// # Composition type
/// Alias of ``Vec<Frg>`` for readability
pub type Composition = Vec<Frg>;

/// # Boxed Fragment trait
/// Alias for the boxed ``Fragment`` trait
pub type Frg = Box<dyn Fragment>;

/// # Composer context
/// Indicates a context as we're iterating over the stream of tokens
#[derive(Debug, PartialEq, Clone)]
pub enum ComposerContext {
    Closure(BracketType),
}

/// # Bracket type
/// Enum for the different types of brackets, such as curly, square
/// and parenthetical
#[derive(Debug, PartialEq, Clone)]
pub enum BracketType {
    Square,
    Curly,
    Parenthetical,
}

/// # Composer
/// The struct used to access the composing functions
#[derive(Debug, Clone)]
pub struct Composer;

impl Composer {
    /// # Compose
    /// Takes a ``LanguageDefinition`` and a ``TokenStream``, and
    /// returns a ``Composition`` struct which contains the hierarchical structure
    /// of the code. To the best of its ability it will also break
    /// the source code into statements.
    pub fn compose(
        langdef: &LanguageDefinition,
        stream: &TokenStream,
    ) -> Composition {
        Self::process(&langdef, &stream)
    }

    /// # Get context
    /// Returns a ``ComposerContext`` based on the passed punctuator.
    /// For example, it maps {...} punctuators to the ``Curly`` type.
    fn get_context(
        punctuator: &TokenType,
    ) -> ComposerContext {
        match punctuator {
            Punctuator('{') | Punctuator('}') => Closure(Curly),
            Punctuator('(') | Punctuator(')') => Closure(Parenthetical),
            Punctuator('[') | Punctuator(']') => Closure(Square),
            _ => panic!("Not a closure type: {:?}", punctuator),
        }
    }

    /// # Process
    /// Take a token stream and language definition, and use those in conjunction
    /// to break a token stream into statements and encapsulations.
    ///
    /// The function returns a ``Composition`` struct which contains sub-elements
    /// which are either a:
    /// - ``Statement``
    /// - ``SingleToken``
    /// - or ``Encapsulation``
    ///
    /// The composition can be used to more easily analyze and evaluate the grammar
    /// and hierarchical structure of the code.
    fn process(
        langdef: &LanguageDefinition,
        stream: &TokenStream,
    ) -> Composition {
        // The overall composition struct which will be returned in the end
        let mut composition: Composition = Composition::new();

        // Indicates when we're looking inside a certain context, such as
        // inside a bracket or parenthetisis
        let mut context: Option<ComposerContext> = None;

        // Temporary container of tokens
        let mut buffer: TokenStream = TokenStream::new();

        // Indicates the indentations of nested closures and their context
        let mut indentations: Vec<ComposerContext> = vec![];

        for token in stream {
            match (&token.token_type, &context) {
                // If we reach the "end of statement" punctuator, we add the current
                // buffer content to the output
                (Punctuator(';'), None) => Self::add_stream(&mut composition, &mut buffer),

                // If we reach the EOS punctuator inside a context, we add it to the buffer
                (Punctuator(';'), _) => buffer.insert(buffer.len(), token.clone()),

                // If we encounter a bracket-style punctuator we open a new context
                (Punctuator('{'), None)
                | (Punctuator('('), None)
                | (Punctuator('['), None) => {
                    Self::add_stream(&mut composition, &mut buffer);
                    context = Some(Self::get_context(&token.token_type));
                },

                // If we encounter a bracket-style punctuator while inside a context,
                // we we just add it to the buffer and indicate in the indentation
                (Punctuator('{'), Some(Closure(_)))
                | (Punctuator('('), Some(Closure(_)))
                | (Punctuator('['), Some(Closure(_))) => {
                    buffer.insert(buffer.len(), token.clone());
                    indentations.insert(indentations.len(), context.clone().unwrap());
                },

                // When reaching the closing punctuator, we insert the obtained buffer
                // content in the output.
                // We recursively handle nested closures, before adding from this layer
                (Punctuator('}'), Some(Closure(_)))
                | (Punctuator(')'), Some(Closure(_)))
                | (Punctuator(']'), Some(Closure(_))) => {
                    if indentations.len() > 0 {
                        buffer.insert(buffer.len(), token.clone());
                        indentations.remove(indentations.len() - 1);
                    } else {
                        Self::add_encapsulation(&langdef, &mut composition, &mut buffer, &mut context);
                    }
                },

                // When none of the above actions are reached, we simply add the token
                // to the buffer
                _ => buffer.insert(buffer.len(), token.clone()),
            }
        }

        // If the buffer holds content, it indicates no token indicated
        // an end of the statement. This is for example seen in nested
        // parenthetical expression such as A + (B + (C + D))
        if buffer.len() > 0 {
            Self::add_stream(&mut composition, &mut buffer);
        }

        composition
    }

    /// # Add encapsulation
    /// Handle the composition, buffer and context
    /// when we trigger the adding of an encapsulation to the composition
    fn add_encapsulation(
        langdef: &LanguageDefinition,
        composition: &mut Composition,
        buffer: &mut TokenStream,
        context: &mut Option<ComposerContext>,
    ) {
        composition.insert(
            composition.len(),
            Box::new(Encapsulation {
                context: context.as_ref().unwrap().clone(),
                composition: Self::process(&langdef, &buffer),
            }),
        );
        *buffer = TokenStream::new();
        *context = None;
    }

    /// # Add token stream to composition
    /// Add the current token stream (granted it has at least one item)
    /// to the composition
    fn add_stream(
        composition: &mut Composition,
        stream: &mut TokenStream,
    ) {
        if stream.len() > 0 {
            composition.insert(
                composition.len(),
                Self::create_fragment(&stream),
            );
        }
        *stream = TokenStream::new();
    }

    /// # Create fragment
    /// Returns the correct Fragment type based on the
    /// number of elements in the token stream
    fn create_fragment(
        stream: &TokenStream,
    ) -> Box<dyn Fragment> {
        match stream.len() {
            1 => Box::new(SingleToken {
                token: stream.get(0).unwrap().clone(),
            }),
            _ => Box::new(Statement {
                token_stream: stream.clone(),
            })
        }
    }
}

impl Debug for dyn Fragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.debug())
    }
}

// Implementation of Clone trait for ``dyn Fragment`` which cannot
// derive Rust's native ``Clone`` trait due to unknown memory size of ``Box``
//
// @todo There's plenty of room for improvement in this code
impl Clone for Box<dyn Fragment> {
    fn clone(&self) -> Box<dyn Fragment> {
        match self.get_type() {
            "single_token" => Box::new(SingleToken {
                token: self.get_token().unwrap().clone(),
            }),
            "block" => Box::new(Encapsulation {
                context: self.get_context().unwrap(),
                composition: self.get_fragments().unwrap(),
            }),
            "statement" => Box::new(Statement {
                token_stream: self.get_tokens().clone(),
            }),
            _ => panic!("Unknown implementation in Clone for Box<dyn Fragment>"),
        }
    }
}
