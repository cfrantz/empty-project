load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def imgui_rs_repos(local = None):
    if local:
        native.local_repository(
            name = "imgui_rs",
            path = local,
        )
    else:
        fail("not ready")
        #http_archive(
        #    name = "imgui_rs",
        #    sha256 = "",
        #    urls = [""],
        #)
