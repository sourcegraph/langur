load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "6357de5982dd32526e02278221bb8d6aa45717ba9bbacf43686b130aa2c72e1e",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.30.0/rules_rust-v0.30.0.tar.gz"],
)

# rust toolchain setup
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains", "rust_repository_set")

rules_rust_dependencies()

rust_version = "1.68.0"

rust_register_toolchains(
    edition = "2021",
    versions = [ rust_version ],
)

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

crates_repository(
    name = "crate_index",
    cargo_config = "//:.cargo/config.toml",
    cargo_lockfile = "//:Cargo.lock",
    # This file has to be manually created and it will be filled when the target is ran.
    # Regenerate using: CARGO_BAZEL_ISOLATED=0 CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index
    lockfile = "//:Cargo.Bazel.lock",
    manifests = ["//:Cargo.toml", "//tools/codegen:Cargo.toml"],
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()
