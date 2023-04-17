load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def pybind11_repos(local = None):
    if local:
        native.new_local_repository(
            name = "pybind11",
            path = local,
            build_file = "//third_party/pybind11:BUILD.pybind11.bazel",
        )
    else:
        http_archive(
            name = "pybind11",
            sha256 = "832e2f309c57da9c1e6d4542dedd34b24e4192ecb4d62f6f4866a737454c9970",
            strip_prefix = "pybind11-2.10.4",
            url = "https://github.com/pybind/pybind11/archive/refs/tags/v2.10.4.tar.gz",
            build_file = "//third_party/pybind11:BUILD.pybind11.bazel",
        )
