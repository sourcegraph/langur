#![allow(dead_code, unused, unused_variables)]

use lazy_static::lazy_static;
use pcre2::bytes::Regex as PCRERegex;
use phf_codegen::Map as PhfMap;
use phf_shared::*;
use regex::Regex;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::{Display, Write as _},
    fs::{self, File},
    hash::{Hash, Hasher},
    io::{BufWriter, Write},
    iter, rc::Rc, path::{Path, PathBuf},
};

const DISAMBIGUATION_HEURISTICS_FILE: &str = "src/generated/disambiguation_heuristics_map.rs";
const EXTENSION_MAP_FILE: &str = "src/generated/extension_language_map.rs";
const FILENAME_MAP_FILE: &str = "src/generated/filename_language_map.rs";
const INTERPRETER_MAP_FILE: &str = "src/generated/interpreter_language_map.rs";
const ALIASES_MAP_FILE: &str = "src/generated/aliases_language_map.rs";
const LANGUAGE_DATA_FILE: &str = "src/generated/language_data_map.rs";
const LANGUAGE_LIST_FILE: &str = "src/generated/languages.rs";
const TOKEN_LOG_PROBABILITY_FILE: &str = "src/generated/token_log_probabilities.rs";

const HEURISTICS_SOURCE_FILE: &str = "lib/linguist/heuristics.yml";
const LANGUAGE_SOURCE_FILE: &str = "lib/linguist/languages.yml";

const MAX_TOKEN_BYTES: usize = 32;

fn main() {
    let linguist_root_dir = PathBuf::from("external/com_github_linguist");
    let parsed_map: ParsedLanguageMap =
        serde_yaml::from_reader(File::open(
            linguist_root_dir.join(LANGUAGE_SOURCE_FILE)
        ).unwrap()).unwrap();
    let language_table = LanguageTable::new(parsed_map, &linguist_root_dir);

    language_table.write_language_list();
    language_table.write_language_data();
    language_table.create_filename_map();
    language_table.create_extension_map();
    language_table.create_interpreter_map();
    language_table.create_aliases_map();

    let heuristics: Heuristics<String> =
        serde_yaml::from_reader(File::open(
            linguist_root_dir.join(HEURISTICS_SOURCE_FILE)
        ).unwrap()).unwrap();

    language_table.create_disambiguation_heuristics_map(heuristics);

    language_table.train_classifier();
}

type ParsedLanguageMap = HashMap<String, ParsedLanguage>;

/// Represents a single language entry in the languages.yml file.
#[derive(Deserialize, Clone)]
struct ParsedLanguage {
    filenames: Option<Vec<String>>,
    interpreters: Option<Vec<String>>,
    extensions: Option<Vec<String>>,
    language_id: i64,
    #[serde(rename(deserialize = "type"))]
    language_type: LanguageType,
    color: Option<String>,
    group: Option<String>,
    aliases: Option<Vec<String>>,
}

impl ParsedLanguage {
    fn id(&self, name: &str) -> LanguageId {
        LanguageId { value: self.language_id, identifier: Rc::new(Identifier::new(name)) }
    }

    fn to_rust_code(&self, name: &str) -> String {
        let mut ident: Identifier;
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
enum LanguageType {
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
    fn to_rust_code(&self) -> String {
        format!("LanguageType::{:?}", self)
    }
}

/// Represents the language_id field stored in Linguist's languages.yml file.
#[derive(Clone, Eq)]
struct LanguageId {
    value: i64,
    identifier: Rc<Identifier>,
}

impl LanguageId {
    fn slice_to_string(ids: &[LanguageId], after_comma: &str) -> String {
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

    fn prefixed_id(&self) -> PrefixedIdentifier<'_> {
        PrefixedIdentifier { identifer: self.identifier.as_ref() }
    }
}

impl PartialEq for LanguageId {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for LanguageId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LanguageId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl Hash for LanguageId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PhfHash for LanguageId {
    fn phf_hash<H: Hasher>(&self, state: &mut H) {
        self.value.phf_hash(state);
    }
}

impl FmtConst for LanguageId {
    fn fmt_const(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.prefixed_id().fmt(f)
    }
}

/// Represents identifiers in codegen, via the Display impl.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Identifier {
    text: String,
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

struct PrefixedIdentifier<'a> {
    identifer: &'a Identifier
}

impl std::fmt::Debug for PrefixedIdentifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn std::fmt::Display).fmt(f)
    }
}

impl Display for PrefixedIdentifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Language::")?;
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
            static ref NON_IDENTIFIER_CHAR_RE: Regex =
                Regex::new("[^a-zA-Z0-9_]").unwrap();
        }
        if s == "C++" {
            return Identifier { text: "Cpp".to_owned() }
        } else if s == "Objective-C++" {
            return Identifier { text: "Objective_Cpp".to_owned() }
        } else if s == "F*" {
            return Identifier { text: "Fstar".to_owned() }
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
        if buf.starts_with(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
            || buf == "Self" {
            buf = format!("Extra_{}", buf);
        }
        Identifier { text: buf }
    }    
}

/// The core data structure representing all the languages
/// in Linguist.
struct LanguageTable {
    /// Core table representing all language data.
    id_to_data_map: HashMap<LanguageId, LanguageDataWithName>,
    /// This is computed once on construction so that languages
    /// can be traversed in a deterministic way for any maps that
    /// need to be generated.
    sorted_names: Vec<(String, LanguageId)>,
    /// Original data from the languages.yml file, kept as-is.
    parsed_map: ParsedLanguageMap,
    linguist_root_dir: PathBuf,
}

impl LanguageTable {
    fn new<P: AsRef<Path>>(parsed_map: ParsedLanguageMap, p: P) -> LanguageTable {
        let mut out = HashMap::with_capacity(parsed_map.len());
        let mut names = Vec::with_capacity(parsed_map.len());
        for (original_name, mut dto) in parsed_map.clone().into_iter() {
            // See https://sourcegraph.com/github.com/github-linguist/linguist@7ca3799b8b5f1acde1dd7a8dfb7ae849d3dfb4cd/-/blob/lib/linguist/languages.yml?L7-8
            // We call to_lowercase explicitly since a bunch of aliases
            // in languages.yml are stored in uppercase, such
            // as "Protocol Buffers"
            let mut v: Vec<_> = dto.aliases.unwrap_or_default().into_iter().map(|s| s.to_lowercase()).collect();
            v.push(original_name.to_lowercase());
            // For 'R', the aliases list contains 'R' too, so de-dupe.
            v.sort();
            v.dedup();
            dto.aliases = Some(v);

            let id = dto.id(&original_name);
            names.push((original_name.clone(), id.clone()));
            let old_value = out.insert(id, LanguageDataWithName { original_name, dto });
            // Invariant 1: All IDs are distinct.
            if let Some(old_data_with_name) = old_value {
                panic!("Language ID: {} is repeated twice", old_data_with_name.dto.language_id);
            }
        }
        names.sort();
        
        let table = LanguageTable { id_to_data_map: out, sorted_names: names, parsed_map, linguist_root_dir: p.as_ref().to_owned() };
        table.check_invariants();
        table
    }

    fn check_invariants(&self) {
        // Invariant 2: No overlaps in aliases.
        {
            let mut alias_map = HashMap::new();
            for (id, lang_data) in self.id_to_data_map.iter() {
                if let Some(aliases) = lang_data.dto.aliases.as_ref() {
                    for alias in aliases.iter() {
                        // Invariant 3: All aliases are lowercased.
                        assert!(alias == &alias.to_lowercase(), "Alias {} for language {} is not lowercase", alias, id.identifier);
                        if let Some(old_id) = alias_map.insert(alias.clone(), id) {
                            panic!("Attempting to add alias {} for language {} but it is already used by {}",
                                alias, id.identifier, old_id.identifier);
                        }
                    }
                }
            }
        }
        // Invariant 4: All identifier names are distinct.
        {
            let mut ident_map = HashMap::new();
            for (lang_name, id) in self.sorted_names.iter() {
                if let Some(old_name) = ident_map.insert(id.identifier.as_ref(), lang_name.clone()) {
                    panic!("Same identifier added for different languages: {old_name} and {lang_name}");
                }
            }
        }
    }

    fn write_language_list(&self) {
        let mut enum_branches = Vec::with_capacity(self.id_to_data_map.len());
        let mut i64_to_id_map = PhfMap::new();
        let mut id_list = Vec::with_capacity(self.id_to_data_map.len());
        for (_, id) in self.sorted_names.iter() {
            enum_branches.push(format!("{} = {}", id.identifier.as_ref(), id.value));
            i64_to_id_map.entry(id.value, &format!("{}", id.prefixed_id()));
            id_list.push(id.clone());
        }
    
        let mut file = BufWriter::new(File::create(LANGUAGE_LIST_FILE).unwrap());
        writeln!(
            &mut file,
"
#[allow(non_camel_case_types)]
#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
/// The core type for identifying Languages in this crate.
/// 
/// This is a non-exhaustive enum rather than being stringly-typed to allow
/// for compiler assistance in catching typos.
/// 
/// The list of languages is based on Linguist's languages.yml file.
/// 
///   https://sourcegraph.com/github.com/github-linguist/linguist/-/blob/lib/linguist/languages.yml
/// 
/// However, that file doesn't make guarantees that languages will not
/// be removed. However, this type will generally not make breaking changes,
/// we will instead mark removed language codes as deprecated.
pub enum Language {{
    {}
}}

// Deliberately private; other modules should use try_from
static I64_TO_LANGUAGE_MAP: phf::Map<i64, Language> =\n{};\n

impl Language {{
    /// List of all variants currently available, for iteration.
    /// 
    /// The exact order of elements is unspecified.
    pub const VARIANTS: &[Language] = 
        {}
    ;
}}
",
            enum_branches.join(",\n    "),
            i64_to_id_map.build(),
            LanguageId::slice_to_string(&id_list, "\n        "),
        )
        .unwrap();
    }

    fn write_language_data(&self) {
        let mut language_info_map = PhfMap::new();
        for (language_name, id) in self.sorted_names.iter() {
            let info = self.id_to_data_map.get(id).unwrap();
            language_info_map.entry(
                id,
                &info.dto.to_rust_code(&language_name[..])[..],
            );
        }
        let built_map = language_info_map.build();
        let mut file = BufWriter::new(File::create(LANGUAGE_DATA_FILE).unwrap());
        writeln!(
            &mut file,
"pub(crate) static LANGUAGE_DATA_MAP: phf::Map<crate::Language, LanguageData> = {{
    use crate::Language;
    {}
}};
",
            built_map,
        )
        .unwrap();    
    }

    /// Create a mapping from filename -> list of language strings.
    ///
    /// For example, HOSTS as a filename is used by both the INI language
    /// and the 'Hosts File' language.
    fn create_filename_map(&self) {
        let mut temp_map: HashMap<&str, Vec<LanguageId>> = HashMap::new();
        for (language_name, id) in self.sorted_names.iter() {
            if let Some(filenames) = &self.id_to_data_map.get(id).unwrap().dto.filenames {
                for filename in filenames.iter() {
                    temp_map.entry(filename).or_default().push(id.clone());
                }
            }
        }

        let mut filename_to_language_map = PhfMap::new();
        for (filename, ids) in temp_map.iter_mut() {
            ids.sort();
            filename_to_language_map.entry(
                filename, &LanguageId::slice_to_string(ids, " "));
        }

        let built_map = filename_to_language_map.build();
        let mut file = BufWriter::new(File::create(FILENAME_MAP_FILE).unwrap());
        writeln!(
            &mut file,
"static FILENAME_TO_LANGUAGE_MAP: phf::Map<&'static str, &[crate::Language]> = {{
    use crate::Language;
    {}
}};
",
            built_map,
        )
        .unwrap();
    }

    fn create_interpreter_map(&self) {
        let mut temp_map: HashMap<&str, Vec<LanguageId>> = HashMap::new();
        for (language_name, id) in self.sorted_names.iter() {
            if let Some(interpreters) = &self.id_to_data_map.get(id).unwrap().dto.interpreters {
                for interpreter in interpreters.iter() {
                    temp_map.entry(interpreter).or_default().push(id.clone());
                }
            }
        }
    
        let mut interpreter_to_language_map = PhfMap::new();
        for (interpreter, ids) in temp_map.iter_mut() {
            ids.sort();
            interpreter_to_language_map.entry(
                interpreter, &LanguageId::slice_to_string(ids, " "));
        }
    
        let built_map = interpreter_to_language_map.build();
        let mut file = BufWriter::new(File::create(INTERPRETER_MAP_FILE).unwrap());
        writeln!(
            &mut file,
"static INTERPRETERS: phf::Map<&'static str, &[crate::Language]> = {{
    use crate::Language;
    {}
}};
",
            built_map,
        )
        .unwrap();
    }
    
    fn create_extension_map(&self) {
        let mut temp_map: HashMap<String, Vec<LanguageId>> = HashMap::new();
        for (language_name, id) in self.sorted_names.iter() {
            if let Some(extensions) = &self.id_to_data_map.get(id).unwrap().dto.extensions {
                for extension in extensions.iter() {
                    let extension = extension.clone().to_ascii_lowercase();
                    temp_map.entry(extension).or_default().push(id.clone());
                }
            }
        }
    
        let mut extension_to_language_map = PhfMap::new();
        for (extension, ids) in temp_map.iter_mut() {
            ids.sort();
            extension_to_language_map.entry(
                extension, &LanguageId::slice_to_string(ids, " "));
        }
    
        let built_map = extension_to_language_map.build();
        let mut file = BufWriter::new(File::create(EXTENSION_MAP_FILE).unwrap());
        writeln!(
            &mut file,
"static EXTENSIONS: phf::Map<&'static str, &[crate::Language]> = {{
    use crate::Language;

    {}
}};
",
            built_map,
        )
        .unwrap();
    }

    fn create_aliases_map(&self) {
        let mut alias_to_language_map = PhfMap::new();
        for (language_name, id) in self.sorted_names.iter() {
            if let Some(aliases) = &self.id_to_data_map.get(id).unwrap().dto.aliases {
                for alias in aliases.iter() {
                    alias_to_language_map.entry(alias, &format!("{}", id.prefixed_id()));
                }
            }
        }

        let built_map = alias_to_language_map.build();
        let mut file = BufWriter::new(File::create(ALIASES_MAP_FILE).unwrap());
        writeln!(
            &mut file,
"static ALIASES_TO_LANGUAGE_MAP: phf::Map<&'static str, &[crate::Language]> = {{
    use crate::Language;

    {}
}};
",
            built_map,
        )
        .unwrap();
    }

    fn train_classifier(&self) {
        let mut temp_token_count: HashMap<LanguageId, HashMap<String, i32>> = HashMap::new();
        let mut temp_total_tokens_count = HashMap::new();
    
        fs::read_dir(self.linguist_root_dir.join("samples"))
            .unwrap()
            .map(|entry| entry.unwrap())
            .filter(|entry| entry.path().is_dir())
            .flat_map(|language_dir| {
                let path = language_dir.path();
                let language = path.file_name().unwrap();
                let language = language.to_string_lossy().into_owned();
                let language = match &language[..] {
                    "Fstar" => String::from("F*"),
                    _ => language,
                };
                let id = self.parsed_map.get(&language)
                    .unwrap_or_else(|| panic!("missing entry for {}", language))
                    .id(&language);
    
                let file_paths = fs::read_dir(language_dir.path())
                    .unwrap()
                    .map(|entry| entry.unwrap().path())
                    .filter(|path| path.is_file());
    
                let id_iter = iter::repeat(id);
                file_paths.zip(id_iter)
            })
            .for_each(|(entry, id)| {
                let content = fs::read(entry).unwrap();
    
                // When tokenizing an invalid utf8 string, just set it to ""
                // Add better error handling here in the future but unure of the best
                // way to handle it now
                let tokens =
                    langur_tokenizer::get_key_tokens(std::str::from_utf8(&content[..]).unwrap_or(""));
    
                for token in tokens {
                    if token.len() <= MAX_TOKEN_BYTES {
                        let total_tokens = temp_total_tokens_count.entry(id.clone()).or_insert(0);
                        *total_tokens += 1;
    
                        let tokens_count = temp_token_count
                            .entry(id.clone())
                            .or_insert(HashMap::new());
    
                        let count = tokens_count.entry(String::from(token)).or_insert(0);
                        *count += 1;
                    }
                }
            });
    
        // Write token log probabilities
        let mut language_token_log_probabilities = PhfMap::new();
        for (id, token_count_map) in temp_token_count.iter() {
            let total_tokens = *temp_total_tokens_count.get(id).unwrap() as f64;
            let mut token_log_probabilities = PhfMap::new();
            for (token, token_count) in token_count_map.iter() {
                let probability = (*token_count as f64) / (total_tokens);
                let log_probability = probability.ln();
                // 8 digits is somewhat arbitrarily chosen to avoid
                // differences across environments.
                token_log_probabilities.entry(&token[..], &format!("{:.8}f64", log_probability)[..]);
            }
            let codegen_log_prob_map = format!("{}", token_log_probabilities.build());
            language_token_log_probabilities.entry(id, &codegen_log_prob_map[..]);
        }
    
        let built_map = language_token_log_probabilities.build();
        let mut file = BufWriter::new(File::create(TOKEN_LOG_PROBABILITY_FILE).unwrap());
        file.write_all("#[allow(clippy::approx_constant)]\n\n".as_bytes()).unwrap();
        writeln!(
            &mut file,
"static TOKEN_LOG_PROBABILITIES: phf::Map<crate::Language, phf::Map<&'static str, f64>> = {{
    use crate::Language;

    {}
}};
",
            built_map,
        )
        .unwrap();
    }    

    fn create_disambiguation_heuristics_map(&self, heuristics: Heuristics<String>) {
        let mut temp_map: HashMap<String, String> = HashMap::new();
        for mut dis in heuristics.disambiguations.into_iter() {
            let rules = dis.rules;
            let extensions = dis.extensions;
            for ext in extensions.iter() {
                let extension = ext.clone().to_ascii_lowercase();
                let key = extension;
                // NOTE(def: special-dot-h-rule)
                // Adding a rule to default to C for .h if the
                // Objective-C and C++ patterns don't match.
                // The classifer was unreliable for distinguishing
                // between C and C++ for .h
                let rules: Vec<_> = 
                    if ext == ".h" {
                        let mut new_rules = rules.clone();
                        new_rules.push(Rule {
                            language: MaybeMany::One(String::from("C")),
                            pattern: None,
                        });
                        new_rules
                    } else {
                        rules.clone()
                    }.into_iter()
                    .map(|r| r.map_language(&|lang_name| {
                        self.parsed_map.get(&lang_name)
                        .unwrap_or_else(|| panic!("missing entry for {}", lang_name))
                        .id(&lang_name)
                    }))
                    .collect();
                let value = Disambiguation::to_rust_code(&rules, &heuristics.named_patterns);
                temp_map.insert(key, value);
            }
        }
    
        let mut disambiguation_heuristic_map = PhfMap::new();
        for (key, value) in temp_map.iter() {
            disambiguation_heuristic_map.entry(&key[..], &value[..]);
        }
    
        let built_map = disambiguation_heuristic_map.build();
        let mut file = BufWriter::new(File::create(DISAMBIGUATION_HEURISTICS_FILE).unwrap());
        writeln!(
            &mut file,
            "static DISAMBIGUATIONS: phf::Map<&'static str, &'static [Rule]> =\n{};\n",
            built_map,
        )
        .unwrap();
    }
}

struct LanguageDataWithName {
    original_name: String,
    dto: ParsedLanguage,
}

type NamedPatterns = HashMap<String, MaybeMany<String>>;

#[derive(Deserialize)]
struct Heuristics<L> {
    disambiguations: Vec<Disambiguation<L>>,
    named_patterns: NamedPatterns,
}

#[derive(Deserialize)]
struct Disambiguation<L> {
    extensions: Vec<String>,
    rules: Vec<Rule<L>>,
}

impl<L> Rule<L> {
    fn map_language<A>(self, f: &dyn Fn(L) -> A) -> Rule<A> {
        Rule {
            language: self.language.map(f),
            pattern: self.pattern,
        }
    }
}

impl Disambiguation<LanguageId> {
    fn to_rust_code(rules: &[Rule<LanguageId>], named_patterns: &NamedPatterns) -> String {
        let mut buf = String::new();
        buf.push_str("&[");
        for rule in rules.iter() {
            buf.push_str(rule.to_rust_code(named_patterns).as_str());
            buf.push(',');
        }
        buf.push(']');
        buf
    }
}

#[derive(Clone, Deserialize)]
struct Rule<L> {
    language: MaybeMany<L>,
    #[serde(flatten)]
    pattern: Option<ParsedPattern>,
}

impl Rule<LanguageId> {
    fn to_rust_code(&self, named_patterns: &NamedPatterns) -> String {
        let ids = match &self.language {
            MaybeMany::Many(values) => values.clone(),
            MaybeMany::One(value) => vec![value.clone()],
        };

        let pattern_code = match &self.pattern {
            Some(pattern) => format!("Some({})", pattern.to_rust_code(named_patterns)),
            None => String::from("None"),
        };

        format!(
            "Rule {{ languages: {}, pattern: {}}}",
            LanguageId::slice_to_string(&ids, " "),
            pattern_code
        )
    }
}

#[derive(Clone, Deserialize)]
enum ParsedPattern {
    #[serde(rename = "and")]
    And(Vec<ParsedPattern>),
    #[serde(rename = "named_pattern")]
    Named(String),
    #[serde(rename = "negative_pattern")]
    Negative(String),
    #[serde(rename = "pattern")]
    Positive(MaybeMany<String>),
}

impl ParsedPattern {
    fn to_rust_code(&self, named_patterns: &NamedPatterns) -> String {
        match self {
            ParsedPattern::Positive(MaybeMany::One(pattern)) => {
                // Panic on invalid regex now so we can unwrap in lib
                if let Err(e) = PCRERegex::new(pattern) {
                    panic!("Invalid regex pattern: {}\n{}", pattern, e);
                }
                format!("Pattern::Positive({:?})", pattern)
            }
            ParsedPattern::Negative(pattern) => {
                // Panic on invalid regex now so we can unwrap in lib
                if let Err(e) = PCRERegex::new(pattern) {
                    panic!("Invalid regex pattern: {}\n{}", pattern, e);
                }
                format!("Pattern::Negative({:?})", pattern)
            }
            ParsedPattern::Positive(MaybeMany::Many(patterns)) => {
                let mut code = String::from("Pattern::Or(&[");
                for pattern in patterns.iter() {
                    let p = ParsedPattern::Positive(MaybeMany::One(pattern.clone()));
                    code.push_str(format!("{},", p.to_rust_code(named_patterns)).as_str());
                }
                code.push_str("])");
                code
            }
            ParsedPattern::And(patterns) => {
                let mut code = String::from("Pattern::And(&[");
                for pattern in patterns.iter() {
                    code.push_str(
                        format!("{},", pattern.to_rust_code(named_patterns)).as_str(),
                    );
                }
                code.push_str("])");
                code
            }
            ParsedPattern::Named(pattern_name) => {
                if let Some(pattern) = named_patterns.get(pattern_name) {
                    // Assume that all named patterns are positive
                    let pattern = ParsedPattern::Positive(pattern.clone());
                    pattern.to_rust_code(named_patterns)
                } else {
                    panic!(
                        "Named pattern: {} not found in named pattern map",
                        pattern_name
                    )
                }
            }
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
enum MaybeMany<T> {
    Many(Vec<T>),
    One(T),
}

impl<T> MaybeMany<T> {
    fn map<A>(self, f: &dyn Fn(T) -> A) -> MaybeMany<A> {
        match self {
            Self::Many(vs) => MaybeMany::Many(vs.into_iter().map(f).collect()),
            Self::One(t) => MaybeMany::One(f(t)),
        }
    }
}