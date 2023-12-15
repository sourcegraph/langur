#![allow(clippy::type_complexity)]

use clap::{App, Arg};
use ignore::{overrides::OverrideBuilder, WalkBuilder};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    env,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::mpsc,
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::{detectors::Detection, filters, Language, LanguageType, LANGUAGE_DATA_MAP};

struct CLIOptions {
    color: bool,
    condensed_output: bool,
    filters: Option<Vec<Regex>>,
}

impl CLIOptions {
    fn matches_filter(&self, pattern: &str) -> bool {
        if let Some(filters) = &self.filters {
            filters.iter().any(|filter| filter.is_match(pattern))
        } else {
            true
        }
    }

    fn color_option(&self) -> ColorChoice {
        if self.color {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        }
    }
}

#[doc(hidden)]
pub fn main() {
    let matches = get_cli().get_matches();
    let path = matches.value_of("PATH").unwrap();
    let breakdown = get_language_breakdown(path);

    let mut language_count: Vec<(Language, Vec<(Detection, PathBuf)>)> = breakdown
        .into_iter()
        .filter(|(language, _)| {
            matches!(
                crate::LANGUAGE_DATA_MAP
                    .get(language)
                    .map(|l| l.language_type),
                Some(LanguageType::Markup) | Some(LanguageType::Programming)
            )
        })
        .collect();
    language_count.sort_by(|(_, a), (_, b)| b.len().cmp(&a.len()));
    if print_language_split(&language_count).is_err() {
        std::process::exit(1);
    }

    let cli_options = CLIOptions {
        color: !matches.is_present("no-color"),
        condensed_output: matches.is_present("condensed"),
        filters: matches
            .values_of("filter")
            .map(|filters| {
                filters.map(|f| Regex::new(f).expect(&format!("Invalid filter: {}", f)[..]))
            })
            .map(|filters| filters.collect()),
    };

    if matches.is_present("file-breakdown") {
        writeln!(io::stdout()).unwrap_or_else(|_| std::process::exit(1));
        if print_file_breakdown(&language_count, &cli_options).is_err() {
            std::process::exit(1);
        }
    }

    if matches.is_present("strategy-breakdown") {
        writeln!(io::stdout()).unwrap_or_else(|_| std::process::exit(1));
        if print_strategy_breakdown(&language_count, &cli_options).is_err() {
            std::process::exit(1);
        }
    }
}

fn get_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("Langur")
        .version("0.1.0")
        .about("Langur is a programming language detector. It supports detecting the programming language of a file or the programming language makeup of a directory.")
        .arg(Arg::with_name("PATH").index(1).default_value("."))
        .arg(
            Arg::with_name("file-breakdown")
                .short("b")
                .long("breakdown")
                .help("prints the language detected for each file visited"),
        )
        .arg(
            Arg::with_name("strategy-breakdown")
                .short("s")
                .long("strategies")
                .help(
                    "Prints each strategy used and what files were detected using that strategy",
                ),
        )
        .arg(
            Arg::with_name("condensed")
                .short("c")
                .long("condensed")
                .help("Condenses the output for the breakdowns to only show the headers"),
        )
        .arg(
            Arg::with_name("filter").short("f").long("filter").help(
                "A regex that is used to filter by header which sections get printed for the file and strategy breakdown.",
            ).takes_value(true).multiple(true),
        )
        .arg(
            Arg::with_name("no-color").short("n").long("no-color").help(
                "Don't color code the output of the breakdowns. This is useful when piping/redirecting the output.",
            ),
        )
}

/// Walks the path provided and tallies the programming languages detected in the given path
///
/// Returns a map from the programming languages to a Vec of the files that were detected and the
/// strategy used
///
/// # Examples
/// ```
/// use langur::get_language_breakdown;
/// let breakdown = get_language_breakdown("src/");
/// let total_detections = breakdown.iter().fold(0, |sum, (language, detections)| sum + detections.len());
/// println!("Total files detected: {}", total_detections);
/// ```
fn get_language_breakdown<P: AsRef<Path>>(path: P) -> HashMap<Language, Vec<(Detection, PathBuf)>> {
    let override_builder = OverrideBuilder::new(&path);
    let override_builder = filters::add_documentation_override(override_builder);
    let override_builder = filters::add_vendor_override(override_builder);

    let num_threads = env::var_os("LANGUR_THREADS")
        .and_then(|threads| threads.into_string().ok())
        .and_then(|threads| threads.parse().ok())
        .unwrap_or_else(num_cpus::get);

    let (tx, rx) = mpsc::channel::<(Detection, PathBuf)>();
    let walker = WalkBuilder::new(path)
        .threads(num_threads)
        .overrides(override_builder.build().unwrap())
        .build_parallel();

    walker.run(|| {
        let tx = tx.clone();
        Box::new(move |result| {
            use ignore::WalkState::*;

            if let Ok(path) = result {
                let path = path.into_path();
                if !path.is_dir() {
                    if let Ok(Some(detection)) = crate::detectors::detect(&path) {
                        tx.send((detection, path)).unwrap();
                    }
                }
            }
            Continue
        })
    });
    drop(tx);

    let mut language_breakdown = HashMap::new();
    for (detection, file) in rx {
        let files = language_breakdown
            .entry(detection.language())
            .or_insert_with(Vec::new);
        files.push((detection, file));
    }

    language_breakdown
}

fn print_language_split(
    language_counts: &[(Language, Vec<(Detection, PathBuf)>)],
) -> Result<(), io::Error> {
    let total = language_counts
        .iter()
        .fold(0, |acc, (_, files)| acc + files.len()) as f64;
    for (language, files) in language_counts.iter() {
        let percentage = ((files.len() * 100) as f64) / total;
        writeln!(
            io::stdout(),
            "{:.2}% {}",
            percentage,
            LANGUAGE_DATA_MAP.get(language).unwrap().name
        )?;
    }

    Ok(())
}

fn print_file_breakdown(
    language_counts: &[(Language, Vec<(Detection, PathBuf)>)],
    options: &CLIOptions,
) -> Result<(), io::Error> {
    let mut stdout = StandardStream::stdout(options.color_option());
    for (language, breakdowns) in language_counts.iter() {
        let language_name = LANGUAGE_DATA_MAP.get(language).unwrap().name;
        if options.matches_filter(language_name) {
            stdout.set_color(&TITLE_COLOR)?;
            write!(stdout, "{}", language_name)?;

            stdout.set_color(&DEFAULT_COLOR)?;
            writeln!(stdout, " ({})", breakdowns.len())?;
            if !options.condensed_output {
                for (_, file) in breakdowns.iter() {
                    let path = strip_relative_parts(file);
                    writeln!(stdout, "{}", path.display())?;
                }
                writeln!(stdout)?;
            }
        }
    }
    Ok(())
}

fn print_strategy_breakdown(
    language_counts: &[(Language, Vec<(Detection, PathBuf)>)],
    options: &CLIOptions,
) -> Result<(), io::Error> {
    let mut strategy_breakdown = HashMap::new();
    for (language, files) in language_counts.iter() {
        let language_name = LANGUAGE_DATA_MAP.get(language).unwrap().name;
        for (detection, file) in files.iter() {
            let files = strategy_breakdown
                .entry(detection.variant())
                .or_insert(BinaryHeap::new());
            files.push(Reverse((language_name, file)));
        }
    }

    let mut strategy_breakdowns: Vec<(&str, BinaryHeap<Reverse<(&str, &PathBuf)>>)> =
        strategy_breakdown.into_iter().collect();
    strategy_breakdowns.sort_by(|(_, a), (_, b)| b.len().cmp(&a.len()));

    let mut stdout = StandardStream::stdout(options.color_option());
    for (strategy, mut breakdowns) in strategy_breakdowns.into_iter() {
        if options.matches_filter(strategy) {
            stdout.set_color(&TITLE_COLOR)?;
            write!(stdout, "{}", strategy)?;

            stdout.set_color(&DEFAULT_COLOR)?;
            writeln!(stdout, " ({})", breakdowns.len())?;
            if !options.condensed_output {
                while let Some(Reverse((language, file))) = breakdowns.pop() {
                    stdout.set_color(&DEFAULT_COLOR)?;
                    let path = strip_relative_parts(file);
                    write!(stdout, "{}", path.display())?;

                    stdout.set_color(&LANGUAGE_COLOR)?;
                    writeln!(stdout, " ({})", language)?;
                }
                writeln!(stdout)?;
            }
        }
    }
    Ok(())
}

fn strip_relative_parts(path: &Path) -> &Path {
    path.strip_prefix("./").unwrap_or(path)
}

lazy_static! {
    static ref TITLE_COLOR: ColorSpec = {
        let mut title_color = ColorSpec::new();
        title_color.set_fg(Some(Color::Magenta));
        title_color
    };
    static ref DEFAULT_COLOR: ColorSpec = ColorSpec::default();
    static ref LANGUAGE_COLOR: ColorSpec = {
        let mut language_color = ColorSpec::new();
        language_color.set_fg(Some(Color::Green));
        language_color
    };
}

#[cfg(test)]
mod tests {
    use super::get_language_breakdown;
    use std::fs;

    #[test]
    fn test_get_language_breakdown_ignores_overrides_documentation() {
        fs::create_dir_all("temp-testing-dir").unwrap();
        fs::File::create("temp-testing-dir/README.md").unwrap();
        assert!(get_language_breakdown("temp-testing-dir").is_empty());

        fs::remove_dir_all("temp-testing-dir").unwrap();
    }

    #[test]
    fn test_get_language_breakdown_ignores_overrides_vendor() {
        fs::create_dir_all("temp-testing-dir2/node_modules").unwrap();
        fs::File::create("temp-testing-dir2/node_modules/hello.go").unwrap();
        assert!(get_language_breakdown("temp-testing-dir2").is_empty());

        fs::remove_dir_all("temp-testing-dir2").unwrap();
    }
}
