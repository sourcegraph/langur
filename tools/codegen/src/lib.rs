use lazy_static::lazy_static;
use phf_shared::*;
use regex::Regex;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::{Display, Write as _},
    fs::File,
    hash::{Hash, Hasher},
    num::NonZeroU8,
    path::PathBuf,
    rc::Rc,
};

pub type ParsedLanguageMap = HashMap<String, ParsedLanguage>;

const LANGUAGE_SOURCE_FILE: &str = "lib/linguist/languages.yml";

pub fn linguist_root_dir() -> PathBuf {
    PathBuf::from("external/com_github_linguist")
}

pub fn parse_languages_yml() -> ParsedLanguageMap {
    serde_yaml::from_reader(File::open(linguist_root_dir().join(LANGUAGE_SOURCE_FILE)).unwrap())
        .unwrap()
}

const DEPRECATED_LANGUAGES_FILE: &str = "tools/codegen/data/deprecated_languages.yml";

pub fn parse_deprecated_languages_yml() -> DeprecatedLanguageMap {
    serde_yaml::from_reader(File::open(PathBuf::from(DEPRECATED_LANGUAGES_FILE)).unwrap()).unwrap()
}

/// Represents a single language entry in the languages.yml file.
#[derive(Deserialize, Clone)]
pub struct ParsedLanguage {
    pub filenames: Option<Vec<String>>,
    pub interpreters: Option<Vec<String>>,
    pub extensions: Option<Vec<String>>,
    pub language_id: i64,
    #[serde(rename(deserialize = "type"))]
    pub language_type: LanguageType,
    pub color: Option<String>,
    pub group: Option<String>,
    pub aliases: Option<Vec<String>>,
}

impl ParsedLanguage {
    pub fn id(&self, name: &str) -> LanguageId {
        LanguageId {
            value: self.language_id,
            deprecated_variant: 0,
            identifier: Rc::new(Identifier::new(name)),
        }
    }

    pub fn deprecated_id(&self, count: NonZeroU8, name: &str) -> LanguageId {
        LanguageId {
            value: self.language_id,
            deprecated_variant: count.get(),
            identifier: Rc::new(Identifier::new(name)),
        }
    }

    pub fn to_rust_code(&self, name: &str) -> String {
        let ident: Identifier;
        let opt_group_id = match &self.group {
            Some(s) => {
                ident = Identifier::new(s);
                Some(PrefixedIdentifier { identifer: &ident })
            }
            None => None,
        };
        format!(
            "LanguageData {{ name: \"{}\", language_type: {}, color: {:?}, group: {:?}, aliases: &{:?} }}",
            name,
            self.language_type.to_rust_code(),
            self.color,
            opt_group_id,
            self.aliases.as_ref().unwrap(),
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum LanguageType {
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "markup")]
    Markup,
    #[serde(rename = "programming")]
    Programming,
    #[serde(rename = "prose")]
    Prose,
}

impl LanguageType {
    pub fn to_rust_code(&self) -> String {
        format!("LanguageType::{:?}", self)
    }
}

pub type DeprecatedLanguageMap = HashMap<String, DeprecatedLanguage>;

#[derive(Deserialize, Clone)]
pub struct DeprecatedLanguage {
    pub replaced_by: String,
}

/// Represents the language_id field stored in Linguist's languages.yml file.
#[derive(Clone, Eq)]
pub struct LanguageId {
    /// The value of the language_id field.
    pub value: i64,
    /// A locally constructed variant number so that even deprecated
    /// languages can be handled as hash map keys.
    deprecated_variant: u8,
    pub identifier: Rc<Identifier>,
}

impl LanguageId {
    pub fn slice_to_string(ids: &[LanguageId], after_comma: &str) -> String {
        let mut buf = String::new();
        buf.push_str("&[");
        for id in ids.iter() {
            buf.write_fmt(format_args!("{}", id.prefixed_id())).unwrap();
            buf.push(',');
            buf.push_str(after_comma);
        }
        buf.push(']');
        buf
    }

    pub fn prefixed_id(&self) -> PrefixedIdentifier<'_> {
        PrefixedIdentifier {
            identifer: self.identifier.as_ref(),
        }
    }

    pub fn is_deprecated(&self) -> bool {
        self.deprecated_variant > 0
    }
}

impl PartialEq for LanguageId {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.deprecated_variant == other.deprecated_variant
    }
}

impl PartialOrd for LanguageId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LanguageId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value
            .cmp(&other.value)
            .then(self.deprecated_variant.cmp(&other.deprecated_variant))
    }
}

impl Hash for LanguageId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.deprecated_variant.hash(state);
    }
}

impl PhfHash for LanguageId {
    fn phf_hash<H: Hasher>(&self, state: &mut H) {
        self.value.phf_hash(state);
        // It is important that only 'value' is inserted
        // for the hash value here, because at runtime,
        // only the 'value' field is available for computing
        // the hash.
        assert!(!self.is_deprecated(), "deprecated languages aren't inserted into perfect hashmaps");
    }
}

impl FmtConst for LanguageId {
    fn fmt_const(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.prefixed_id().fmt(f)
    }
}

/// Represents identifiers in codegen, via the Display impl.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    pub text: String,
}

impl std::fmt::Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn std::fmt::Display).fmt(f)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

pub struct PrefixedIdentifier<'a> {
    pub identifer: &'a Identifier,
}

impl std::fmt::Debug for PrefixedIdentifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn std::fmt::Display).fmt(f)
    }
}

impl Display for PrefixedIdentifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ids::")?;
        self.identifer.fmt(f)
    }
}

impl FmtConst for Identifier {
    fn fmt_const(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Identifier {
    fn new(mut s: &str) -> Identifier {
        // WARNING: Changes to this function may break public facing API,
        // so be extra careful!
        lazy_static! {
            static ref NON_IDENTIFIER_CHAR_RE: Regex = Regex::new("[^a-zA-Z0-9_]").unwrap();
        }
        if s == "C++" {
            return Identifier {
                text: "Cpp".to_owned(),
            };
        } else if s == "Objective-C++" {
            return Identifier {
                text: "Objective_Cpp".to_owned(),
            };
        } else if s == "F*" {
            return Identifier {
                text: "Fstar".to_owned(),
            };
        }
        let mut add_sharp = false;
        if let Some(suffix) = s.strip_suffix('#') {
            s = suffix;
            add_sharp = true;
        }
        let mut buf = NON_IDENTIFIER_CHAR_RE.replace_all(s, "_").to_string();
        if add_sharp {
            buf.push_str("Sharp");
        }
        // Cannot use a raw identifier here
        // https://internals.rust-lang.org/t/raw-identifiers-dont-work-for-all-identifiers/9094
        if buf.starts_with(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']) || buf == "Self" {
            buf = format!("Extra_{}", buf);
        }
        Identifier { text: buf }
    }
}
