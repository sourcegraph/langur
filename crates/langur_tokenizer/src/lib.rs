pub mod tokenizer;
pub use tokenizer::{Token, Tokenizer};

/// Tokenize the content and return only the identifiers and symbols from the langauge
///
/// # Examples
/// ```
/// use langur_tokenizer;
/// let content = r#"let x = [5, "hello"];"#;
/// let tokens: Vec<&str> = langur_tokenizer::get_key_tokens(content).collect();
/// assert_eq!(tokens, vec!["let", "x", "=", "[", ",", "]", ";"]);
/// ```
#[doc(hidden)]
pub fn get_key_tokens(content: &str) -> impl Iterator<Item = &str> {
    Tokenizer::new(content).tokens().filter_map(|t| match t {
        Token::Ident(t) | Token::Symbol(t) => Some(t),
        _ => None,
    })
}
