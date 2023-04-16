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
            sha256 = "950a3ad4166ae60c8ccd628d1a8e64396106e7f98361ebe91b0bcfe60d8e4b60",
            urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.20.0/rules_rust-v0.20.0.tar.gz"],
        )
