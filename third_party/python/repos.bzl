load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def python_repos(local = None):
    if local:
        native.local_repository(
            name = "rules_python",
            path = local,
        )
    else:
        http_archive(
            name = "rules_python",
            sha256 = "a644da969b6824cc87f8fe7b18101a8a6c57da5db39caa6566ec6109f37d2141",
            strip_prefix = "rules_python-0.20.0",
            url = "https://github.com/bazelbuild/rules_python/releases/download/0.20.0/rules_python-0.20.0.tar.gz",
        )
