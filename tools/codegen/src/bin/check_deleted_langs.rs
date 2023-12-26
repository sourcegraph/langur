use codegen::{parse_deprecated_languages_yml, parse_languages_yml, ParsedLanguageMap};

fn main() {
    let linguist_version = std::env::var("NEW_LINGUIST_TAG")
        .expect("Set NEW_LINGUIST_TAG to check for updates");
    let url = format!(
        "https://raw.githubusercontent.com/github-linguist/linguist/{}/lib/linguist/languages.yml",
        linguist_version
    );
    let mut resp = reqwest::blocking::get(&url).expect("failed to download language.yml file");
    let mut body = Vec::new();
    resp.copy_to(&mut body).unwrap();
    let body = String::from_utf8(body).unwrap();

    let current_languages_yml = parse_languages_yml();

    let new_languages_yml: ParsedLanguageMap = serde_yaml::from_reader(body.as_bytes()).unwrap();
    let deprecated_languages = parse_deprecated_languages_yml();

    let mut deleted_languages = Vec::new();
    for (language_name, _) in current_languages_yml {
        if !new_languages_yml.contains_key(&language_name)
            && !deprecated_languages.contains_key(&language_name)
        {
            deleted_languages.push(language_name.clone());
        }
    }

    if deleted_languages.is_empty() {
        return;
    }

    eprintln!(
        "The following languages are removed from the Linguist repo in {linguist_version}
but haven't been added to deprecated_languages.yml:\n{}",
        deleted_languages.join("\n")
    );
    std::process::exit(1);
}
