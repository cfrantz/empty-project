# Copyright lowRISC contributors.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

load("//rules:repo.bzl", "http_archive_or_local")

def google_repos(
        absl = None,
        googletest = None,
        protobuf = None):
    http_archive_or_local(
        name = "com_google_absl",
        local = absl,
        sha256 = "59d2976af9d6ecf001a81a35749a6e551a335b949d34918cfade07737b9d93c5",
        strip_prefix = "abseil-cpp-20230802.0",
        url = "https://github.com/abseil/abseil-cpp/archive/refs/tags/20230802.0.tar.gz",
    )

    http_archive_or_local(
        name = "googletest",
        local = googletest,
        sha256 = "353571c2440176ded91c2de6d6cd88ddd41401d14692ec1f99e35d013feda55a",
        strip_prefix = "googletest-release-1.11.0",
        url = "https://github.com/google/googletest/archive/refs/tags/release-1.11.0.zip",
    )

    http_archive_or_local(
        name = "com_google_protobuf",
        local = protobuf,
        #sha256 = "c29d8b4b79389463c546f98b15aa4391d4ed7ec459340c47bffe15db63eb9126",
        strip_prefix = "protobuf-25.1",
        url = "https://github.com/protocolbuffers/protobuf/archive/refs/tags/v25.1.tar.gz",
    )

    http_archive_or_local(
        name = "rules_proto",
        local = None,
        sha256 = "dc3fb206a2cb3441b485eb1e423165b231235a1ea9b031b4433cf7bc1fa460dd",
        strip_prefix = "rules_proto-5.3.0-21.7",
        urls = [
            "https://github.com/bazelbuild/rules_proto/archive/refs/tags/5.3.0-21.7.tar.gz",
        ],
    )
