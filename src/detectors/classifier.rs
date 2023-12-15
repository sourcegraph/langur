// Include the map that contains the token log probabilities
// static TOKEN_LOG_PROBABILITIES: phf::Map<Language, f64> = ...;
include!("../generated/token_log_probabilities.rs");

// Include the array of all possible languages

const MAX_TOKEN_BYTES: usize = 32;
const DEFAULT_LOG_PROB: f64 = -19f64;

use crate::Language;

#[derive(Debug)]
struct LanguageScore {
    language: Language,
    score: f64,
}

/// Pre-condition: !candidates.is_empty()
pub(crate) fn classify(content: &str, candidates: &[Language]) -> Language {
    assert!(
        !candidates.is_empty(),
        "classify requires 1 or more candidates"
    );

    let tokens: Vec<_> = langur_tokenizer::get_key_tokens(content)
        .filter(|token| token.len() <= MAX_TOKEN_BYTES)
        .collect();

    let mut scored_candidates: Vec<LanguageScore> = candidates
        .iter()
        .map(|&language| {
            let score = match TOKEN_LOG_PROBABILITIES.get(&language) {
                Some(token_map) => tokens
                    .iter()
                    .map(|token| token_map.get(*token).copied().unwrap_or(DEFAULT_LOG_PROB))
                    .sum(),
                None => std::f64::NEG_INFINITY,
            };
            LanguageScore { language, score }
        })
        .collect();

    scored_candidates.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    scored_candidates[0].language
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Language as L;
    use std::fs;
    use std::path::PathBuf;

    fn linguist_path(s: &str) -> PathBuf {
        PathBuf::from("external/com_github_linguist").join(s)
    }

    #[test]
    fn test_classify() {
        let content = fs::read_to_string(linguist_path("samples/Rust/main.rs")).unwrap();
        let candidates = &[L::C, L::Rust];
        let language = classify(content.as_str(), candidates);
        assert_eq!(language, L::Rust);

        let content =
            fs::read_to_string(linguist_path("samples/Erlang/170-os-daemons.es")).unwrap();
        let candidates = &[L::Erlang, L::JavaScript];
        let language = classify(content.as_str(), candidates);
        assert_eq!(language, L::Erlang);

        let content = fs::read_to_string(linguist_path("samples/TypeScript/classes.ts")).unwrap();
        let candidates = &[L::Cpp, L::Java, L::CSharp, L::TypeScript];
        let language = classify(content.as_str(), candidates);
        assert_eq!(language, L::TypeScript);
    }

    #[test]
    fn test_classify_non_sample_data() {
        let sample = r#"#[cfg(not(feature = "pcre2"))]
    fn imp(args: &Args) -> Result<bool> {
        let mut stdout = args.stdout();
        writeln!(stdout, "PCRE2 is not available in this build of ripgrep.")?;
        Ok(false)
    }

    imp(args)"#;
        let candidates = &[L::Rust, L::RenderScript];
        let language = classify(sample, candidates);
        assert_eq!(language, L::Rust);
    }

    #[test]
    fn test_classify_empty_and_all_candidates() {
        let content = fs::read_to_string(linguist_path("samples/Rust/main.rs")).unwrap();
        let candidates = &[];
        assert!(std::panic::catch_unwind(|| { classify(content.as_str(), candidates) }).is_err());
        let candidates = L::VARIANTS;
        let language = classify(content.as_str(), candidates);
        assert_eq!(language, L::Rust);
    }

    #[test]
    fn test_classify_f_star() {
        let content = fs::read_to_string(linguist_path("samples/Fstar/Hacl.HKDF.fst")).unwrap();
        let candidates = L::VARIANTS;
        let language = classify(content.as_str(), candidates);
        assert_eq!(language, L::Fstar);
    }
}
