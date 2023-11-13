# Langur - Fast language detection library & CLI

Langur is work-in-progress Rust library and CLI for detecting programming and markup languages
based on GitHub's [Linguist](https://github.com/github/linguist) data.

It started off as a fork of [Hyperpolyglot](https://github.com/monkslc/hyperpolyglot)
(commit: a55a3b58eaed09b4314ef93d78e50a80cfec36f4), but has been modified
to use Bazel as the build system to help systematically manage multiple language
toolchains as we add bindings for other languages, as well as multi-language
benchmarks comparing Langur and [go-enry](https://github.com/go-enry/go-enry).
The original README is available under [OLD_README.md](./OLD_README.md).
