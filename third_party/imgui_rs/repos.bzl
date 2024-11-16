load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def imgui_rs_repos(local = None):
    if local:
        native.local_repository(
            name = "imgui_rs",
            path = local,
        )
    else:
        http_archive(
            name = "imgui_rs",
            sha256 = "5b696aaebcaaa50227c548acb4a66fc50c73a1cfacd09388cceab56e910045cb",
            url = "https://github.com/imgui-rs/imgui-rs/archive/refs/tags/v0.12.0.tar.gz",
            strip_prefix = "imgui-rs-0.12.0",
            patch_args = ["-p1"],
            patches = [
                "@//third_party/imgui_rs/patches:bazel-support.patch",
            ],
        )
