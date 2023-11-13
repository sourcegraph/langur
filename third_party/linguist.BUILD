filegroup(
    name = "samples",
    visibility = ["//visibility:public"],
    # Bazel build files must not be interpreted as sub-packages
    # so that subdirectory is included in .bazelignore
    #
    # However, glob doesn't allow finding matches in .bazelignore-d
    # directories, so just list the files directly here as a workaround.
    # (See https://github.com/bazelbuild/bazel/issues/12034)
    #
    # We hit a similar issue in the SG monorepo earlier.
    # (See https://github.com/sourcegraph/sourcegraph/pull/58146)
    srcs = glob(["samples/**/*"], allow_empty=False)
      + [
         "samples/Starlark/filenames/BUCK",
         "samples/Starlark/filenames/BUILD",
         "samples/Starlark/filenames/BUILD.bazel",
         "samples/Starlark/filenames/WORKSPACE",
         # Disabled because we're using an older commit, not master
         # "samples/Starlark/filenames/MODULE.bazel",
         # "samples/Starlark/filenames/Tiltfile",
         # "samples/Starlark/filenames/WORKSPACE.bazel",
      ],
)

filegroup(
    name = "yaml_config",
    visibility = ["//visibility:public"],
    srcs = glob(["lib/linguist/*.yml"], allow_empty=False),
)
