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
            sha256 = "0fcdc7f8952801db9c6bb41bb298d33325c7db6b26ae123df9b28de97a46fb56",
            url = "https://github.com/imgui-rs/imgui-rs/archive/refs/tags/v0.11.0.tar.gz",
            strip_prefix = "imgui-rs-0.11.0",
            patch_args = ["-p1"],
            patches = [
                "@//third_party/imgui_rs/patches:bazel-support.patch",
            ],
        )
