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

# _LINGUIST_VERSION = "7.27.0"
# http_archive(
#     name = "com_github_linguist",
#     sha256 = "3d2c2f5bce2af68a52bc300c2e4991145ad201826b0c73358e89127d85e30d0e",
#     urls = ["https://github.com/github-linguist/linguist/archive/refs/tags/v%s.tar.gz" % _LINGUIST_VERSION],
#     strip_prefix = "linguist-%s" % _LINGUIST_VERSION,
#     build_file = "//third_party:linguist.BUILD",
#     patches = ["//third_party:linguist.patch"],
# )

# With master + .bazelignore
# _FORK_LINGUIST_VERSION = "a05ca8a4b0bc03b29a3e5f6808a586872ec657f9"

# With old commit as used by hyperpolyglot + .bazelignore
_FORK_LINGUIST_VERSION = "51731a48490586d726eaae5e8bfb38b15d1b3238"

http_archive(
    name = "com_github_linguist",
    sha256 = "cae178fce3d4e6de913d8781bbf91ea1da9ec846a479cd64bd47b67872ce6186",
    build_file = "//third_party:linguist.BUILD",
    strip_prefix = "linguist-%s" % _FORK_LINGUIST_VERSION,
    urls = ["https://github.com/varungandhi-src/linguist/archive/%s.tar.gz" % _FORK_LINGUIST_VERSION],
    # .bazelignore was added to my fork, so this patch isn't needed
    # patches = ["//third_party:linguist.patch"],
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
        "//crates/polyglot_tokenizer:Cargo.toml",
        "//tools/codegen:Cargo.toml",
    ],
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()
