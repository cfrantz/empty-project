load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def rust_repos(local = None):
    if local:
        native.local_repository(
            name = "rules_rust",
            path = local,
        )
    else:
        http_archive(
            name = "rules_rust",
            integrity = "sha256-r09Wyq5QqZpov845sUG1Cd1oVIyCBLmKt6HK/JTVuwI=",
            urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.54.1/rules_rust-v0.54.1.tar.gz"],
        )
