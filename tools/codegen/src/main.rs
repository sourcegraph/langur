#![allow(dead_code, unused, unused_variables)]

use lazy_static::lazy_static;
use pcre2::bytes::Regex as PCRERegex;
use phf_codegen::Map as PhfMap;
use phf_shared::*;
use regex::Regex;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufWriter, Write},
    iter, rc::Rc,
    hash::{Hash, Hasher}, fmt::{Display, Write as _},
};

#[derive(Clone, Eq)]
struct LanguageId {
    value: i64,
    identifier: Rc<Identifier>,
}

impl LanguageId {
    fn slice_to_string(ids: &[LanguageId]) -> String {
        let mut buf = String::new();
        buf.push_str("&[");
        for id in ids.iter() {
            buf.push_str("Language::");
            buf.write_fmt(format_args!("{}", id.identifier.as_ref())).unwrap();
            buf.push(',');
        }
        buf.push(']');
        buf
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
        f.write_str("Language::")?;
        self.identifier.fmt_const(f)
    }
}

impl FmtConst for Identifier {
    fn fmt_const(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

type ParsedLanguageMap = HashMap<String, LanguageDTO>;
type NamedPatterns = HashMap<String, MaybeMany<String>>;

struct LanguageTable {
    id_to_data_map: HashMap<LanguageId, LanguageDataWithName>,
    sorted_names: Vec<(String, LanguageId)>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Identifier {
    text: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Cannot use a raw identifier here
        // https://internals.rust-lang.org/t/raw-identifiers-dont-work-for-all-identifiers/9094
        if self.text.starts_with(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
            || self.text == "Self" {
            f.write_fmt(format_args!("Extra_{}", &self.text))
        } else {
            f.write_str(&self.text)
        }
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
        let mut add_sharp = false;
        if let Some(suffix) = s.strip_suffix('#') {
            s = suffix;
            add_sharp = true;
        }
        let mut buf = NON_IDENTIFIER_CHAR_RE.replace_all(s, "_").to_string();
        if add_sharp {
            buf.push_str("Sharp");
        }
        Identifier { text: buf }
    }    
}

impl LanguageTable {
    fn new(parsed_map: ParsedLanguageMap) -> LanguageTable {
        let mut out = HashMap::with_capacity(parsed_map.len());
        let mut names = Vec::with_capacity(parsed_map.len());
        for (original_name, dto) in parsed_map.into_iter() {
            let identifier = Rc::new(Identifier::new(&original_name));
            let id = LanguageId { value: dto.language_id, identifier: identifier.clone() };
            names.push((original_name.clone(), id.clone()));
            let old_value = out.insert(id, LanguageDataWithName { original_name, dto });
            if let Some(old_data_with_name) = old_value {
                panic!("Language ID: {} is repeated twice", old_data_with_name.dto.language_id);
            }
        }
        names.sort();
        LanguageTable { id_to_data_map: out, sorted_names: names }
    }
}

struct LanguageDataWithName {
    original_name: String,
    dto: LanguageDTO,
}

#[derive(Deserialize)]
struct LanguageDTO {
    filenames: Option<Vec<String>>,
    interpreters: Option<Vec<String>>,
    extensions: Option<Vec<String>>,
    language_id: i64,
    #[serde(rename(deserialize = "type"))]
    language_type: LanguageType,
    color: Option<String>,
    group: Option<String>,
}

impl LanguageDTO {
    fn to_domain_object_code(&self, name: &str) -> String {
        format!(
            "LanguageData {{ name: \"{}\", language_type: {}, color: {:?}, group: {:?} }}",
            name,
            self.language_type.to_domain_object_code(),
            self.color,
            self.group
        )
    }
}

#[derive(Deserialize, Debug)]
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
    fn to_domain_object_code(&self) -> String {
        format!("LanguageType::{:?}", self)
    }
}

#[derive(Deserialize)]
struct Heuristics {
    disambiguations: Vec<Disambiguation>,
    named_patterns: NamedPatterns,
}

#[derive(Deserialize)]
struct Disambiguation {
    extensions: Vec<String>,
    rules: Vec<RuleDTO>,
}

impl Disambiguation {
    fn to_domain_object_code(&self, named_patterns: &NamedPatterns) -> String {
        let mut rules = String::new();
        for rule in self.rules.iter() {
            rules.push_str(format!("{},", rule.to_domain_object_code(named_patterns)).as_str());
        }
        format!("&[{}]", rules)
    }
}

#[derive(Deserialize)]
struct RuleDTO {
    language: MaybeMany<String>,
    #[serde(flatten)]
    pattern: Option<PatternDTO>,
}

impl RuleDTO {
    fn to_domain_object_code(&self, named_patterns: &NamedPatterns) -> String {
        let languages = match &self.language {
            MaybeMany::Many(values) => values.clone(),
            MaybeMany::One(value) => vec![value.clone()],
        };

        let pattern_code = match &self.pattern {
            Some(pattern) => format!("Some({})", pattern.to_domain_object_code(named_patterns)),
            None => String::from("None"),
        };

        format!(
            "Rule {{ languages: &[\"{}\"], pattern: {}}}",
            languages.join("\",\""),
            pattern_code
        )
    }
}

#[derive(Clone, Deserialize)]
enum PatternDTO {
    #[serde(rename = "and")]
    And(Vec<PatternDTO>),
    #[serde(rename = "named_pattern")]
    Named(String),
    #[serde(rename = "negative_pattern")]
    Negative(String),
    #[serde(rename = "pattern")]
    Positive(MaybeMany<String>),
}

impl PatternDTO {
    fn to_domain_object_code(&self, named_patterns: &NamedPatterns) -> String {
        match self {
            PatternDTO::Positive(MaybeMany::One(pattern)) => {
                // Panic on invalid regex now so we can unwrap in lib
                if let Err(e) = PCRERegex::new(pattern) {
                    panic!("Invalid regex pattern: {}\n{}", pattern, e);
                }
                format!("Pattern::Positive({:?})", pattern)
            }
            PatternDTO::Negative(pattern) => {
                // Panic on invalid regex now so we can unwrap in lib
                if let Err(e) = PCRERegex::new(pattern) {
                    panic!("Invalid regex pattern: {}\n{}", pattern, e);
                }
                format!("Pattern::Negative({:?})", pattern)
            }
            PatternDTO::Positive(MaybeMany::Many(patterns)) => {
                let mut code = String::from("Pattern::Or(&[");
                for pattern in patterns.iter() {
                    let p = PatternDTO::Positive(MaybeMany::One(pattern.clone()));
                    code.push_str(format!("{},", p.to_domain_object_code(named_patterns)).as_str());
                }
                code.push_str("])");
                code
            }
            PatternDTO::And(patterns) => {
                let mut code = String::from("Pattern::And(&[");
                for pattern in patterns.iter() {
                    code.push_str(
                        format!("{},", pattern.to_domain_object_code(named_patterns)).as_str(),
                    );
                }
                code.push_str("])");
                code
            }
            PatternDTO::Named(pattern_name) => {
                if let Some(pattern) = named_patterns.get(pattern_name) {
                    // Assume that all named patterns are positive
                    let pattern = PatternDTO::Positive(pattern.clone());
                    pattern.to_domain_object_code(named_patterns)
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

const DISAMBIGUATION_HEURISTICS_FILE: &str = "src/generated/disambiguation_heuristics_map.rs";
const EXTENSION_MAP_FILE: &str = "src/generated/extension_language_map.rs";
const FILENAME_MAP_FILE: &str = "src/generated/filename_language_map.rs";
const INTERPRETER_MAP_FILE: &str = "src/generated/interpreter_language_map.rs";
const LANGUAGE_DATA_FILE: &str = "src/generated/language_data_map.rs";
const LANGUAGE_LIST_FILE: &str = "src/generated/languages.rs";
const TOKEN_LOG_PROBABILITY_FILE: &str = "src/generated/token_log_probabilities.rs";

const HEURISTICS_SOURCE_FILE: &str = "external/com_github_linguist/lib/linguist/heuristics.yml";
const LANGUAGE_SOURCE_FILE: &str = "external/com_github_linguist/lib/linguist/languages.yml";
const SAMPLES_DIR: &str = "external/com_github_linguist/samples";

const MAX_TOKEN_BYTES: usize = 32;

fn main() {
    let parsed_map: ParsedLanguageMap =
        serde_yaml::from_reader(File::open(LANGUAGE_SOURCE_FILE).unwrap()).unwrap();
    let language_table = LanguageTable::new(parsed_map);

    language_table.write_language_list();
    language_table.write_language_data();
    language_table.create_filename_map();
    language_table.create_extension_map();
    language_table.create_interpreter_map();

    let heuristics: Heuristics =
        serde_yaml::from_str(&fs::read_to_string(HEURISTICS_SOURCE_FILE).unwrap()[..]).unwrap();
    create_disambiguation_heuristics_map(heuristics);

    train_classifier();
}

impl LanguageTable {
    fn write_language_list(&self) {
        let mut enum_branches = Vec::with_capacity(self.id_to_data_map.len());
        let mut i64_to_id_map = PhfMap::new();
        for (_, id) in self.sorted_names.iter() {
            enum_branches.push(format!("{} = {}", id.identifier.as_ref(), id.value));
            i64_to_id_map.entry(id.value, &format!("Language::{}", id.identifier.as_ref()));
        }
    
        let mut file = BufWriter::new(File::create(LANGUAGE_LIST_FILE).unwrap());
        writeln!(
            &mut file,
"
#[allow(non_camel_case_types)]
#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Language {{
    {}
}}

impl TryFrom<i64> for Language {{
    type Error = ();
    fn try_from(id: i64) -> Result<Self, Self::Error> {{
        match I64_TO_LANGUAGE_MAP.get(&id) {{
            Some(language) => Ok(*language),
            None => Err(()),
        }}
    }}
}}

// Deliberately private; other modules should use try_from
static I64_TO_LANGUAGE_MAP: phf::Map<i64, Language> =\n{};\n
",
            // "static LANGUAGES: &[&str] = &[\n    \"{}\"\n];",
            enum_branches.join(",\n    "),
            i64_to_id_map.build(),
        )
        .unwrap();
    }

    fn write_language_data(&self) {
        let mut language_info_map = PhfMap::new();
        for (language_name, id) in self.sorted_names.iter() {
            let info = self.id_to_data_map.get(id).unwrap();
            language_info_map.entry(
                id,
                &info.dto.to_domain_object_code(&language_name[..])[..],
            );
        }
        let built_map = language_info_map.build();
        let mut file = BufWriter::new(File::create(LANGUAGE_DATA_FILE).unwrap());
        writeln!(
            &mut file,
            "
static LANGUAGE_DATA_MAP: phf::Map<Language, LanguageData> =
{};
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
                filename, &LanguageId::slice_to_string(ids));
        }

        let built_map = filename_to_language_map.build();
        let mut file = BufWriter::new(File::create(FILENAME_MAP_FILE).unwrap());
        writeln!(
            &mut file,
            "
use crate::Language;

static FILENAME_TO_LANGUAGE_MAP: phf::Map<&'static str, &[Language]> =
{};
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
                interpreter, &LanguageId::slice_to_string(ids));
        }
    
        let built_map = interpreter_to_language_map.build();
        let mut file = BufWriter::new(File::create(INTERPRETER_MAP_FILE).unwrap());
        writeln!(
            &mut file,
            "
use crate::Language;

static INTERPRETERS: phf::Map<&'static str, &[Language]> =
{};
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
                extension, &LanguageId::slice_to_string(ids));
        }
    
        let built_map = extension_to_language_map.build();
        let mut file = BufWriter::new(File::create(EXTENSION_MAP_FILE).unwrap());
        writeln!(
            &mut file,
            "
use crate::Language;

static EXTENSIONS: phf::Map<&'static str, &[Language]> =
{};
",
            built_map,
        )
        .unwrap();
    }
}


fn create_disambiguation_heuristics_map(heuristics: Heuristics) {
    let mut temp_map: HashMap<String, String> = HashMap::new();
    for mut dis in heuristics.disambiguations.into_iter() {
        for ext in dis.extensions.iter() {
            // Adding a rule to default to C for .h if the Objective C and C++ patterns don't match
            // The classifer was unreliable for distinguishing between C and C++ for .h
            if ext == ".h" {
                dis.rules.push(RuleDTO {
                    language: MaybeMany::One(String::from("C")),
                    pattern: None,
                });
            }
            let extension = ext.clone().to_ascii_lowercase();
            let key = extension;
            let value = dis.to_domain_object_code(&heuristics.named_patterns);
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

fn train_classifier() {
    let mut temp_token_count: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut temp_total_tokens_count = HashMap::new();

    fs::read_dir(SAMPLES_DIR)
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

            let file_paths = fs::read_dir(language_dir.path())
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.is_file());

            let language_iter = iter::repeat(language);
            file_paths.zip(language_iter)
        })
        .for_each(|(entry, language)| {
            let content = fs::read(entry).unwrap();

            // When tokenizing an invalid utf8 string, just set it to ""
            // Add better error handling here in the future but unure of the best
            // way to handle it now
            let tokens =
                langur_tokenizer::get_key_tokens(std::str::from_utf8(&content[..]).unwrap_or(""));

            for token in tokens {
                if token.len() <= MAX_TOKEN_BYTES {
                    let total_tokens = temp_total_tokens_count.entry(language.clone()).or_insert(0);
                    *total_tokens += 1;

                    let tokens_count = temp_token_count
                        .entry(language.clone())
                        .or_insert(HashMap::new());

                    let count = tokens_count.entry(String::from(token)).or_insert(0);
                    *count += 1;
                }
            }
        });

    // Write token log probabilities
    let mut language_token_log_probabilities = PhfMap::new();
    for (language, token_count_map) in temp_token_count.iter() {
        let total_tokens = *temp_total_tokens_count.get(language).unwrap() as f64;
        let mut token_log_probabilities = PhfMap::new();
        for (token, token_count) in token_count_map.iter() {
            let probability = (*token_count as f64) / (total_tokens);
            let log_probability = probability.ln();
            // 8 digits is somewhat arbitrarily chosen to avoid
            // differences across environments.
            token_log_probabilities.entry(&token[..], &format!("{:.8}f64", log_probability)[..]);
        }
        let codegen_log_prob_map = format!("{}", token_log_probabilities.build());
        language_token_log_probabilities.entry(&language[..], &codegen_log_prob_map[..]);
    }

    let built_map = language_token_log_probabilities.build();
    let mut file = BufWriter::new(File::create(TOKEN_LOG_PROBABILITY_FILE).unwrap());
    file.write_all("#[allow(clippy::approx_constant)]\n\n".as_bytes()).unwrap();
    writeln!(
        &mut file,
        "static TOKEN_LOG_PROBABILITIES: phf::Map<&'static str, phf::Map<&'static str, f64>> =\n{};\n",
        built_map,
    )
    .unwrap();
}
