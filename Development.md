# Developing Langur

## Running tests

Run:

```sh
bazel test //... --test_output=errors
```

## Code generation

After making changes to the codegen script, run:

```sh
bazel run //tools/codegen
```

If you're changing the names of generated files, you may
need to run `bazel clean`, otherwise you may get an error like:

```
ERROR: <dir>/langur/tools/codegen/BUILD.bazel:4:12: Creating runfiles tree bazel-out/darwin_arm64-fastbuild/bin/tools/codegen/codegen.runfiles failed: java.io.IOException: /private/var/tmp/_bazel_me/d282a0949840151ed1c694b0d149ee43/execroot/__main__/bazel-out/darwin_arm64-fastbuild/bin/tools/codegen/codegen.runfiles/__main__/src/generated/my_new_file.rs (File exists)
```

## Adding new Rust crate dependencies

Edit the appropriate `Cargo.toml` file(s) and run:

```sh
CARGO_BAZEL_ISOLATED=0 CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index
```

## Updating Linguist version

If you want to update to a new version of Linguist, first
check if any languages are removed in it.

```sh
# Example with the new tag
NEW_LINGUIST_TAG=v7.28.0 bazel run //tools/codegen:check_deleted_langs
```

This will print a list of languages that should be added to
[deprecated_languages.yml](/tools/codegen/data/deprecated_languages.yml).

Generally, languages are renamed or merged into other languages;
in such a case, track down the associated PRs and add appropriate
entries to `deprecated_languages.yml`.[^1]

[^1]: For other changes, you may need to tweak the schema
in `deprecated_languages.yml` and adjust the codegen.

After that, update the version tag and SHA in the Bazel configuration,
and re-run the [code generation step](#code-generation).