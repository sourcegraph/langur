load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "bazel_skylib",
    sha256 = "cd55a062e763b9349921f0f5db8c3933288dc8ba4f76dd9416aac68acee3cb94",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.5.0/bazel-skylib-1.5.0.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.5.0/bazel-skylib-1.5.0.tar.gz",
    ],
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

bazel_skylib_workspace()

http_archive(
    name = "rules_rust",
    sha256 = "6357de5982dd32526e02278221bb8d6aa45717ba9bbacf43686b130aa2c72e1e",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.30.0/rules_rust-v0.30.0.tar.gz"],
)

_LINGUIST_VERSION = "7.28.0"
http_archive(
    name = "com_github_linguist",
    sha256 = "57d0a9e5139cd4d209023256b28e7e8ca1b61cae3e7cafcff015094f9669f700",
    urls = ["https://github.com/github-linguist/linguist/archive/refs/tags/v%s.tar.gz" % _LINGUIST_VERSION],
    strip_prefix = "linguist-%s" % _LINGUIST_VERSION,
    build_file = "//third_party:linguist.BUILD",
    patches = [
        "//third_party:linguist-bazelignore.patch",
    ],
)

# rust toolchain setup
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

# Keep version in sync with .github/workflows/ci.yml
rust_version = "1.68.0"

rust_register_toolchains(
    edition = "2021",
    versions = [rust_version],
)

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

crates_repository(
    name = "crate_index",
    cargo_config = "//:.cargo/config.toml",
    cargo_lockfile = "//:Cargo.lock",
    # This file has to be manually created and it will be filled when the target is ran.
    # Regenerate using: CARGO_BAZEL_ISOLATED=0 CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index
    lockfile = "//:Cargo.Bazel.lock",
    manifests = [
        "//:Cargo.toml",
        "//crates/langur_tokenizer:Cargo.toml",
        "//tools/codegen:Cargo.toml",
    ],
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()
