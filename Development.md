# Developing Langur

## Code generation

After making changes to the codegen script, run:

```bash
bazel run //tools/codegen
```

If you're changing the names of generated files, you may
need to run `bazel clean`, otherwise you may get an error like:

```
ERROR: <dir>/langur/tools/codegen/BUILD.bazel:4:12: Creating runfiles tree bazel-out/darwin_arm64-fastbuild/bin/tools/codegen/codegen.runfiles failed: java.io.IOException: /private/var/tmp/_bazel_me/d282a0949840151ed1c694b0d149ee43/execroot/__main__/bazel-out/darwin_arm64-fastbuild/bin/tools/codegen/codegen.runfiles/__main__/src/generated/my_new_file.rs (File exists)
```
