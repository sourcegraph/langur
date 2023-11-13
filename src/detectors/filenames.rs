// Include the map from filenames to languages at compile time
// static FILENAMES: phf::Map<&'static str, &'static str> = ...;
include!("../generated/filename_language_map.rs");

pub fn get_language_from_filename(filename: &str) -> Option<&'static str> {
    if let Some(slice) = FILENAMES.get(filename) {
        if slice.len() == 1 {
            return Some(slice[0])
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_language_from_filename() {
        assert_eq!(
            get_language_from_filename("APKBUILD"),
            Some("Alpine Abuild")
        );
        assert_eq!(
            get_language_from_filename(".eslintrc.json"),
            Some("JSON with Comments")
        );
    }
}
