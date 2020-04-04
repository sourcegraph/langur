use std::{error::Error, io::Read};

// Include the map that counts tokens per language
// static LANGUAGE_TOKEN_COUNT: phf::Map<&'static str, f64> = ...;
include!(concat!(env!("OUT_DIR"), "/language_token_count.rs"));

// Include the map that counts the total number of tokens for a language
// static TOTAL_TOKEN_COUNT: phf::Map<&'static str, f64> = ...;
include!(concat!(env!("OUT_DIR"), "/total_token_count.rs"));

// Include the array of all possible languages
// static LANGUAGES: &[&'static str] = ...;
include!(concat!(env!("OUT_DIR"), "/languages.rs"));

#[derive(Debug)]
pub struct LanguageScore {
    language: &'static str,
    score: f64,
}

pub fn classify<R: Read>(
    mut reader: R,
    candidates: &Vec<&'static str>,
) -> Result<&'static str, Box<dyn Error>> {
    let candidates = match candidates.len() {
        0 => LANGUAGES,
        _ => candidates,
    };

    // Return error if invalid utf-8 for now
    // Add better invalid utf-8 support in the future
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let tokens = tokens::tokenize(content.as_str())?;
    let mut scored_candidates: Vec<LanguageScore> = candidates
        .iter()
        .map(|language| {
            let score = tokens.iter().fold(0f64, |sum, token| {
                let token_prob = token_probability(language, token).ln();
                sum + token_prob
            });
            LanguageScore {
                language: language,
                score,
            }
        })
        .collect();

    scored_candidates.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(scored_candidates[0].language)
}

fn token_probability(language: &str, token: &str) -> f64 {
    let key = format!("{}{}", language, token);
    let count = LANGUAGE_TOKEN_COUNT.get(&key[..]).unwrap_or(&1E-5f64);

    // Can't just unwrap here because there are languages in the languages.yml
    // file that we don't have samples for and therefore no tokens have been seen
    let total = TOTAL_TOKEN_COUNT.get(language).unwrap_or(&1f64);
    count / total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Cursor;

    #[test]
    fn test_classify() {
        let f = File::open("samples/Rust/main.rs").unwrap();
        let candidates = vec!["C", "Rust"];
        let language = classify(f, &candidates).unwrap();
        assert_eq!(language, "Rust");

        let f = File::open("samples/Erlang/170-os-daemons.es").unwrap();
        let candidates = vec!["Erlang", "JavaScript"];
        let language = classify(f, &candidates).unwrap();
        assert_eq!(language, "Erlang");

        let f = File::open("samples/TypeScript/classes.ts").unwrap();
        let candidates = vec!["C++", "Java", "C#", "TypeScript"];
        let language = classify(f, &candidates).unwrap();
        assert_eq!(language, "TypeScript");
    }

    #[test]
    fn test_classify_non_sample_data() {
        let sample = Cursor::new(
            r#"#[cfg(not(feature = "pcre2"))]
    fn imp(args: &Args) -> Result<bool> {
        let mut stdout = args.stdout();
        writeln!(stdout, "PCRE2 is not available in this build of ripgrep.")?;
        Ok(false)
    }

    imp(args)"#,
        );
        let candidates = vec!["Rust", "C", "C++"];
        let language = classify(sample, &candidates).unwrap();
        assert_eq!(language, "Rust");
    }

    #[test]
    fn test_classify_empty_candidates() {
        let f = File::open("samples/Rust/main.rs").unwrap();
        let candidates = vec![];
        let language = classify(f, &candidates).unwrap();
        assert_eq!(language, "Rust");
    }
}