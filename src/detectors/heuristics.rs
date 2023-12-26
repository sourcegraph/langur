use pcre2::bytes::RegexBuilder as PCRERegex;

// Include the map from interpreters to languages at compile time
// static DISAMBIGUATIONS: phf::Map<&'static str, &'static [Rule]> = ...;
include!("../generated/disambiguation_heuristics_map.rs");

#[derive(Debug)]
enum Pattern {
    And(&'static [Pattern]),
    Negative(&'static str),
    Or(&'static [Pattern]),
    Positive(&'static str),
}

use crate::{ids, Language};

#[derive(Debug)]
struct Rule {
    languages: &'static [Language],
    pattern: Option<Pattern>,
}

impl Pattern {
    fn matches(&self, content: &str) -> bool {
        match self {
            Pattern::Positive(pattern) => {
                let regex = PCRERegex::new()
                    .crlf(true)
                    .multi_line(true)
                    .build(pattern)
                    .unwrap();
                regex.is_match(content.as_bytes()).unwrap_or(false)
            }
            Pattern::Negative(pattern) => {
                let regex = PCRERegex::new()
                    .crlf(true)
                    .multi_line(true)
                    .build(pattern)
                    .unwrap();
                !regex.is_match(content.as_bytes()).unwrap_or(true)
            }
            Pattern::Or(patterns) => patterns.iter().any(|pattern| pattern.matches(content)),
            Pattern::And(patterns) => patterns.iter().all(|pattern| pattern.matches(content)),
        }
    }
}

pub(crate) fn get_languages_from_heuristics(
    extension: &str,
    candidates: &[Language],
    content: &str,
) -> Vec<Language> {
    match DISAMBIGUATIONS.get(extension) {
        Some(rules) => {
            let rules = rules.iter().filter(|rule| {
                rule.languages
                    .iter()
                    .all(|language| candidates.contains(language))
            });
            for rule in rules {
                if let Some(pattern) = &rule.pattern {
                    if pattern.matches(content) {
                        return rule.languages.to_vec();
                    };
                } else {
                    // if there is no pattern then it is a match by default
                    return rule.languages.to_vec();
                };
            }
            vec![]
        }
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids;

    #[test]
    fn test_heuristics_get_languages_positive_pattern() {
        assert_eq!(
            get_languages_from_heuristics(".es", &[ids::Erlang, ids::JavaScript], "'use strict';"),
            vec![ids::JavaScript]
        );
    }

    #[test]
    fn test_heuristics_get_languages_negative_pattern() {
        assert_eq!(
            get_languages_from_heuristics(
                ".sql",
                &[ids::PLSQL, ids::PLpgSQL, ids::SQL, ids::SQLPL, ids::TSQL],
                "LALA THIS IS SQL"
            ),
            vec![ids::SQL]
        );
    }

    #[test]
    fn test_heuristics_get_languages_and_positives_pattern() {
        assert_eq!(
            get_languages_from_heuristics(
                ".pro",
                &[ids::Proguard, ids::Prolog, ids::INI, ids::QMake, ids::IDL],
                "HEADERS SOURCES"
            ),
            vec![ids::QMake]
        );
    }

    #[test]
    fn test_heuristics_get_languages_and_not_all_match() {
        assert_eq!(
            get_languages_from_heuristics(
                ".pro",
                &[ids::Proguard, ids::Prolog, ids::INI, ids::QMake, ids::IDL],
                "HEADERS"
            ),
            vec![]
        );
    }

    #[test]
    fn test_heuristics_get_languages_and_negative_pattern() {
        assert_eq!(
            get_languages_from_heuristics(
                ".ms",
                &[ids::Roff, ids::Unix_Assembly, ids::MAXScript],
                ".include:"
            ),
            vec![ids::Unix_Assembly]
        );
    }

    #[test]
    fn test_heuristics_get_languages_or_pattern() {
        assert_eq!(
            get_languages_from_heuristics(".p", &[ids::Gnuplot, ids::OpenEdge_ABL], "plot"),
            vec![ids::Gnuplot]
        );
    }

    #[test]
    fn test_heuristics_get_languages_named_pattern() {
        assert_eq!(
            get_languages_from_heuristics(".h", &[ids::Objective_C, ids::Cpp], "std::out"),
            vec![ids::Cpp]
        );
    }

    #[test]
    fn test_heuristics_get_languages_default_pattern() {
        assert_eq!(
            get_languages_from_heuristics(".man", &[ids::Roff_Manpage, ids::Roff], "alskdjfahij"),
            vec![ids::Roff]
        );
    }

    #[test]
    fn test_heuristics_get_languages_multiple_anchors() {
        assert_eq!(
            get_languages_from_heuristics(
                ".1in",
                &[ids::Roff_Manpage, ids::Roff],
                r#".TH LYXCLIENT 1 "@LYX_DATE@" "Version @VERSION@" "lyxclient @VERSION@"
.SH NAME"#
            ),
            vec![ids::Roff_Manpage]
        );
    }
}