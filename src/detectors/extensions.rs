// Include the map from extensions to languages at compile time
// static EXTENSIONS: phf::Map<&'static str, &[&str]> = ...;
include!("../generated/extension_language_map.rs");

use crate::Language;

pub(crate) fn get_languages_from_extension(extension: &str) -> Vec<Language> {
    let languages = EXTENSIONS
        .get(extension)
        .map(|languages| languages.to_vec());

    match languages {
        Some(languages) => languages,
        None => vec![],
    }
}

pub(crate) fn get_extension(filename: &str) -> Option<&'static str> {
    let filename = if let Some(filename_no_dot) = filename.strip_prefix('.') {
        filename_no_dot
    } else {
        filename
    };

    let filename = filename.to_ascii_lowercase();
    for (pos, ch) in filename.char_indices() {
        if ch == '.' {
            if let Some(extension) = EXTENSIONS.get_key(&filename[pos..]) {
                return Some(extension);
            };
        };
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids;

    #[test]
    fn test_get_languages_from_extension() {
        assert_eq!(get_languages_from_extension(".djs"), vec![ids::Dogescript]);
        assert_eq!(get_languages_from_extension(".cmake.in"), vec![ids::CMake]);

        let mut header_file_langs = get_languages_from_extension(".h");
        header_file_langs.sort();
        assert_eq!(header_file_langs, vec![ids::C, ids::Cpp, ids::Objective_C]);

        assert_eq!(get_languages_from_extension(""), vec![]);
    }

    #[test]
    fn test_get_extension() {
        assert_eq!(get_extension("index.djs"), Some(".djs"));
        assert_eq!(get_extension("example.cmake.in"), Some(".cmake.in"));
        assert_eq!(get_extension("nonsense.notrealextension.c"), Some(".c"));
        assert_eq!(get_extension("uppercase.C"), Some(".c"));
        assert_eq!(get_extension(".eslintrc.json"), Some(".json"));
        assert_eq!(get_extension(".cs"), None);
        assert_eq!(get_extension("noextension"), None);
    }
}
