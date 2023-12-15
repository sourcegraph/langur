mod classifier;
mod extensions;
mod filenames;
mod heuristics;
mod interpreters;

use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};

use crate::Language;

/// An enum where the variant is the strategy that detected the language and the value is the name
/// of the language
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Detection {
    Filename(Language),
    Extension(Language),
    Shebang(Language),
    Heuristics(Language),
    Classifier(Language),
}

impl Detection {
    /// Returns the language detected
    pub(crate) fn language(&self) -> Language {
        match self {
            Detection::Filename(language)
            | Detection::Extension(language)
            | Detection::Shebang(language)
            | Detection::Heuristics(language)
            | Detection::Classifier(language) => *language,
        }
    }

    /// Returns the strategy used to detect the langauge
    pub(crate) fn variant(&self) -> &str {
        match self {
            Detection::Filename(_) => "Filename",
            Detection::Extension(_) => "Extension",
            Detection::Shebang(_) => "Shebang",
            Detection::Heuristics(_) => "Heuristics",
            Detection::Classifier(_) => "Classifier",
        }
    }
}

fn filter_candidates<L: PartialEq + Copy>(
    previous_candidates: Vec<L>,
    new_candidates: Vec<L>,
) -> Vec<L> {
    if previous_candidates.is_empty() {
        return new_candidates;
    }

    if new_candidates.is_empty() {
        return previous_candidates;
    }

    let filtered_candidates: Vec<_> = previous_candidates
        .iter()
        .filter(|l| new_candidates.contains(l))
        .copied()
        .collect();

    match filtered_candidates.len() {
        0 => previous_candidates,
        _ => filtered_candidates,
    }
}

/// Detects the programming language of the file at a given path
///
/// If the language cannot be determined, None will be returned.
/// `detect` will error on an io error or if the parser returns an error when tokenizing the
/// contents of the file
///
/// # Examples
/// ```
/// let path = Path::new("src/bin/main.rs");
/// let language = detect(path).unwrap().unwrap();
/// assert_eq!(Detection::Heuristics(Language::Rust), language);
/// ```
pub(crate) fn detect(path: &Path) -> Result<Option<Detection>, std::io::Error> {
    let filename = match path.file_name() {
        Some(filename) => filename.to_str(),
        None => return Ok(None),
    };

    let candidate = filename.and_then(filenames::get_language_from_filename);
    if let Some(candidate) = candidate {
        return Ok(Some(Detection::Filename(candidate)));
    };

    let extension = filename.and_then(extensions::get_extension);

    let candidates = extension
        .map(extensions::get_languages_from_extension)
        .unwrap_or_else(Vec::new);

    if candidates.len() == 1 {
        return Ok(Some(Detection::Extension(candidates[0])));
    };

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let candidates = filter_candidates(
        candidates,
        interpreters::get_languages_from_shebang(&mut reader)?,
    );
    if candidates.len() == 1 {
        return Ok(Some(Detection::Shebang(candidates[0])));
    };
    reader.seek(SeekFrom::Start(0))?;

    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    const MAX_CONTENT_SIZE_BYTES: usize = 51200;

    let content = truncate_to_char_boundary(&content, MAX_CONTENT_SIZE_BYTES);

    // using heuristics is only going to be useful if we have more than one candidate
    // if the extension didn't result in candidate languages then the heuristics won't either
    let candidates: Vec<Language> = if candidates.len() > 1 {
        if let Some(extension) = extension {
            let languages =
                heuristics::get_languages_from_heuristics(extension, &candidates, content);
            filter_candidates(candidates, languages)
        } else {
            candidates
        }
    } else {
        candidates
    };

    match candidates.len() {
        0 => Ok(None),
        1 => Ok(Some(Detection::Heuristics(candidates[0]))),
        _ => Ok(Some(Detection::Classifier(classifier::classify(
            content,
            &candidates,
        )))),
    }
}

// function stolen from from https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html
fn truncate_to_char_boundary(s: &str, mut max: usize) -> &str {
    if max >= s.len() {
        s
    } else {
        while !s.is_char_boundary(max) {
            max -= 1;
        }
        &s[..max]
    }
}

#[cfg(test)]
mod tests {
    use super::filter_candidates;

    use super::*;
    use std::fs;
    use std::io::prelude::*;
    use std::iter;
    use std::path::PathBuf;

    #[test]
    fn test_detect_filename() {
        let path = Path::new("APKBUILD");
        let detected_language = detect(path).unwrap().unwrap();

        assert_eq!(
            detected_language,
            Detection::Filename(Language::Alpine_Abuild)
        );
    }

    #[test]
    fn test_detect_extension() {
        let path = Path::new("pizza.purs");
        let detected_language = detect(path).unwrap().unwrap();

        assert_eq!(
            detected_language,
            Detection::Extension(Language::PureScript)
        );
    }

    #[test]
    fn test_detect_shebang() {
        let path = Path::new("a");
        let mut file = File::create(path).unwrap();
        file.write_all(b"#!/usr/bin/python").unwrap();
        file.flush().unwrap();

        let detected_language = detect(path).unwrap().unwrap();

        fs::remove_file(path).unwrap();

        assert_eq!(detected_language, Detection::Shebang(Language::Python));
    }

    #[test]
    fn test_detect_heuristics() {
        let path = Path::new("a.es");
        let mut file = File::create(path).unwrap();
        file.write_all(b"'use strict'").unwrap();
        file.flush().unwrap();

        let detected_language = detect(path).unwrap().unwrap();

        fs::remove_file(path).unwrap();

        assert_eq!(
            detected_language,
            Detection::Heuristics(Language::JavaScript)
        );
    }

    #[test]
    fn test_detect_classify() {
        let path = Path::new("peep.rs");
        let mut file = File::create(path).unwrap();
        file.write_all(
            b"
            match optional {
                Some(pattern) => println!(\"Hello World\"),
                None => println!(\"u missed\")
            }
            ",
        )
        .unwrap();
        file.flush().unwrap();

        let detected_language = detect(path).unwrap().unwrap();

        fs::remove_file(path).unwrap();
        assert_eq!(detected_language, Detection::Classifier(Language::Rust));
    }

    #[test]
    fn test_detect_none() {
        let path = Path::new("y");
        let mut file = File::create(path).unwrap();
        file.write_all(
            b"
            use std::io;
            fn main() {
                println!(\"{}\", \"Hello World\");
            }",
        )
        .unwrap();
        file.flush().unwrap();

        let detected_language = detect(path).unwrap();

        fs::remove_file(path).unwrap();

        assert_eq!(detected_language, None);
    }

    fn linguist_path(s: &str) -> PathBuf {
        PathBuf::from("external/com_github_linguist").join(s)
    }

    #[test]
    fn test_detect_accuracy() {
        let mut total = 0;
        let mut correct = 0;
        fs::read_dir(linguist_path("samples"))
            .unwrap()
            .map(|entry| entry.unwrap())
            .filter(|entry| entry.path().is_dir())
            .flat_map(|language_dir| {
                let lang_folder_name = language_dir
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();

                let file_paths = fs::read_dir(language_dir.path())
                    .unwrap()
                    .map(|entry| entry.unwrap().path())
                    .filter(|path| path.is_file());

                file_paths.zip(iter::repeat(lang_folder_name))
            })
            .for_each(|(file, folder_name)| {
                // Skip the files we can't detect. The reason the detect function
                // fails on these is because of a heuristic added to .h files
                // that defaults to C if none of the Objective-C or C++ rules
                // match. This makes us fail on two of the sample files
                // but tends to perform better on non training data
                //
                // See NOTE(ref: special-dot-h-rule)
                if file.file_name().unwrap() == "rpc.h" || file.file_name().unwrap() == "Field.h" {
                    return;
                }
                // F* uses the name Fstar in the file system
                // This is covered by the fs_name field in languages.yml,
                // but we don't emit that field in the generated Rust code.
                let language_name_from_folder_name = match &folder_name[..] {
                    "Fstar" => "F*",
                    l => l,
                };
                if let Ok(Some(detection)) = detect(&file) {
                    total += 1;
                    let language_name = crate::LANGUAGE_DATA_MAP
                        .get(&detection.language())
                        .unwrap()
                        .name;
                    if language_name == language_name_from_folder_name {
                        correct += 1;
                    } else {
                        println!("Incorrect detection: {:?} {:?}", file, detection)
                    }
                }
            });

        let accuracy = (correct as f64) / (total as f64);
        assert_eq!(accuracy, 1.0);
    }

    #[test]
    fn test_filter_candidates() {
        let previous_candidates = vec!["JavaScript", "Python"];
        let new_candidates = vec!["Python", "Bibbity"];
        assert_eq!(
            filter_candidates(previous_candidates, new_candidates),
            vec!["Python"]
        );
    }

    #[test]
    fn test_filter_candidates_no_new() {
        let previous_candidates = vec!["JavaScript", "Python"];
        let new_candidates = vec![];
        assert_eq!(
            filter_candidates(previous_candidates, new_candidates),
            vec!["JavaScript", "Python"]
        );
    }

    #[test]
    fn test_filter_candidates_no_prev() {
        let previous_candidates = vec![];
        let new_candidates = vec!["JavaScript", "Erlang"];
        assert_eq!(
            filter_candidates(previous_candidates, new_candidates),
            vec!["JavaScript", "Erlang"]
        );
    }

    #[test]
    fn test_filter_candidates_no_matches() {
        let previous_candidates = vec!["Python"];
        let new_candidates = vec!["JavaScript", "Erlang"];
        assert_eq!(
            filter_candidates(previous_candidates, new_candidates),
            vec!["Python"]
        );
    }
}
