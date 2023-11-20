use phf_shared::{PhfBorrow, PhfHash};
use std::convert::TryFrom;
use std::hash::Hasher;
use std::fmt;

use crate::LanguageData;

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

// Include the map that stores language info
// static LANGUAGE_DATA_MAP: phf::Map<crate::Language, LanguageData> = ...;
include!("generated/language_data_map.rs");

/// The set of possible language types
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LanguageType {
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