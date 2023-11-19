mod classifier;
mod extensions;
mod filenames;
mod heuristics;
mod interpreters;

pub(crate) use classifier::classify;
pub(crate) use extensions::{get_extension, get_languages_from_extension};
pub(crate) use filenames::get_language_from_filename;
pub(crate) use heuristics::get_languages_from_heuristics;
pub(crate) use interpreters::get_languages_from_shebang;
