/// # Language Definition
/// The struct describing the rules and looks of the language
/// to be tokenized and interpreted
#[derive(Debug, Clone, PartialEq)]
pub struct LanguageDefinition {
    keywords: Vec<&'static str>,
}

impl LanguageDefinition {
    /// # New language definition
    /// Create a ``LanguageDefinition`` struct
    pub fn new(
        keywords: Vec<&'static str>,
    ) -> LanguageDefinition {
        LanguageDefinition {
            keywords,
        }
    }

    /// # Has keyword
    /// Returns true, if the ``keyword`` parameter is defined
    /// as a keyword in the language definition
    pub fn has_keyword(&self, keyword: &String) -> bool {
        self.keywords.contains(&&**keyword)
    }
}
