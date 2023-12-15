//! # Langur
//! `langur` is a fast programming language detector.

use phf_shared::{PhfBorrow, PhfHash};
use std::{convert::TryFrom, fmt, hash::Hasher};

mod detectors;
mod filters;

#[doc(hidden)]
pub mod cli;

include!("generated/languages.rs");

impl PhfBorrow<Language> for Language {
    fn borrow(&self) -> &Language {
        self
    }
}

impl PhfHash for Language {
    fn phf_hash<H: Hasher>(&self, state: &mut H) {
        (*self as i64).phf_hash(state)
    }
}

impl TryFrom<i64> for Language {
    type Error = ();
    fn try_from(id: i64) -> Result<Self, Self::Error> {
        match I64_TO_LANGUAGE_MAP.get(&id) {
            Some(language) => Ok(*language),
            None => Err(()),
        }
    }
}

/// The set of possible language types
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum LanguageType {
    Data,
    Markup,
    Programming,
    Prose,
}

impl fmt::Display for LanguageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LanguageType::Data => write!(f, "Data"),
            LanguageType::Markup => write!(f, "Markup"),
            LanguageType::Programming => write!(f, "Programming"),
            LanguageType::Prose => write!(f, "Prose"),
        }
    }
}

/// The language struct that contains the name and other interesting information about a
/// language.
///
/// # Examples
/// ```
/// use langur::{Language, LanguageType};
/// use std::convert::TryFrom;
///
/// let language = Language::try_from("Rust").unwrap();
/// let expected = Language {
///     name: "Rust",
///     language_type: LanguageType::Programming,
///     color: Some("#dea584"),
///     group: None,
/// };
/// assert_eq!(language, expected)
/// ```
///
/// # Errors
/// `try_from` will error if the langauge name is not one of the known languages
///
/// If try_from is called with a language returned from [`detect`] or [`get_language_breakdown`]
/// the value is guaranteed to be there and can be unwrapped
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LanguageData {
    /// The name of the language
    pub name: &'static str,
    /// Categorization for the language.
    language_type: LanguageType,
    /// The css hex color used to represent the language on github. For example, "#dea584".
    pub color: Option<&'static str>,
    /// Name of the parent language. For example, the group for TSX would be TypeScript.
    pub group: Option<Language>,
    /// Non-empty list of aliases allowed for this language.
    ///
    /// Always contains at least the lowercased name of the language.
    aliases: &'static [&'static str],
}

include!("generated/language_data_map.rs");
