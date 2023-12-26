// Include the map from filenames to languages at compile time
// static FILENAME_TO_LANGUAGE_MAP: phf::Map<&'static str, &'static str> = ...;
include!("../generated/filename_language_map.rs");

pub(crate) fn get_language_from_filename(filename: &str) -> Option<crate::Language> {
    if let Some(slice) = FILENAME_TO_LANGUAGE_MAP.get(filename) {
        if slice.len() == 1 {
            return Some(slice[0]);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids;

    #[test]
    fn test_get_language_from_filename() {
        assert_eq!(
            get_language_from_filename("APKBUILD"),
            Some(ids::Alpine_Abuild)
        );
        assert_eq!(
            get_language_from_filename(".eslintrc.json"),
            Some(ids::JSON_with_Comments)
        );
    }
}
