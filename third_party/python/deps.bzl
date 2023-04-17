load("@rules_python//python:repositories.bzl", "py_repositories")
load("@rules_python//python:repositories.bzl", "python_register_toolchains")

def python_deps():
    py_repositories()
    python_register_toolchains(
        name = "python3",
        python_version = "3.10.9",
    )
